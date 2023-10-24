use std::collections::HashSet;
use std::str::FromStr;

use futures_util::StreamExt;
use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    Database,
};
use serde::{Deserialize, Serialize, Serializer};
use surrealdb::{
    engine::remote::ws::Client as SurrealClient,
    sql::{Id, Thing},
    Surreal,
};

mod account;
mod batch;
mod branch;
mod contact;
mod desktop_client;
mod discount_code;
mod doctor;
mod financial_year;
mod gst_registration;
mod inventory;
mod manufacturer;
mod member;
mod patient;
mod pharma_salt;
mod pos_terminal;
mod print_template;
mod rack;
mod sale_incharge;
mod save_voucher;
mod section;
mod set_account_opening;
mod set_inventory_opening;
mod tds_nature_of_payment;
mod unit;
mod vendor_bill_mapping;
mod vendor_item_mapping;
mod voucher_numbering;
mod voucher_type;

pub use account::Account;
pub use batch::Batch;
pub use branch::Branch;
pub use contact::Contact;
pub use desktop_client::DesktopClient;
pub use discount_code::DiscountCode;
pub use doctor::Doctor;
pub use financial_year::FinancialYear;
pub use gst_registration::GstRegistration;
pub use inventory::*;
pub use manufacturer::Manufacturer;
pub use member::Member;
pub use patient::Patient;
pub use pharma_salt::PharmaSalt;
pub use pos_terminal::PosTerminal;
pub use print_template::PrintTemplate;
pub use rack::Rack;
pub use sale_incharge::SaleIncharge;
pub use save_voucher::*;
pub use section::Section;
pub use set_account_opening::AccountOpening;
pub use set_inventory_opening::InventoryOpening;
pub use tds_nature_of_payment::TdsNatureOfPayment;
pub use unit::Unit;
pub use vendor_bill_mapping::VendorBillMapping;
pub use vendor_item_mapping::VendorItemMapping;
pub use voucher_numbering::VoucherNumbering;
pub use voucher_type::VoucherType;

fn serialize_opt_tax_as_thing<S: Serializer>(
    val: &Option<String>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(tax_name) => {
            let tax_id = match tax_name.as_str() {
                "gstna" => "gst_tax:not_applicable",
                "gstexempt" => "gst_tax:exempt",
                "gstngs" => "gst_tax:non_gst_supply",
                "gst0" => "gst_tax:0",
                "gst0p1" => "gst_tax:0_1",
                "gst0p25" => "gst_tax:0_25",
                "gst1" => "gst_tax:1",
                "gst1p5" => "gst_tax:1_5",
                "gst3" => "gst_tax:3",
                "gst5" => "gst_tax:5",
                "gst7p5" => "gst_tax:7_5",
                "gst12" => "gst_tax:12",
                "gst18" => "gst_tax:18",
                "gst28" => "gst_tax:28",
                _ => unreachable!(),
            };
            Thing::from_str(tax_id).unwrap().serialize(ser)
        }
        _ => unreachable!(),
    }
}

fn serialize_tax_as_thing<S: Serializer>(val: &str, ser: S) -> Result<S::Ok, S::Error> {
    let tax_id = match val {
        "gstna" => "gst_tax:not_applicable",
        "gstexempt" => "gst_tax:exempt",
        "gstngs" => "gst_tax:non_gst_supply",
        "gst0" => "gst_tax:0",
        "gst0p1" => "gst_tax:0_1",
        "gst0p25" => "gst_tax:0_25",
        "gst1" => "gst_tax:1",
        "gst1p5" => "gst_tax:1_5",
        "gst3" => "gst_tax:3",
        "gst5" => "gst_tax:5",
        "gst7p5" => "gst_tax:7_5",
        "gst12" => "gst_tax:12",
        "gst18" => "gst_tax:18",
        "gst28" => "gst_tax:28",
        _ => unreachable!(),
    };
    Thing::from_str(tax_id).unwrap().serialize(ser)
}

#[derive(Deserialize, Clone)]
pub struct Created {
    pub id: Thing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AmountInfo {
    pub mode: char,
    pub amount: f64,
}

impl Default for AmountInfo {
    fn default() -> Self {
        Self {
            mode: 'P',
            amount: 0.0,
        }
    }
}

impl From<AmountInfo> for Bson {
    fn from(info: AmountInfo) -> Self {
        to_bson(&info).unwrap()
    }
}

#[derive(Debug, Serialize)]

pub struct GstInfo {
    pub reg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_no: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BillAllocationApiInput {
    pub sno: usize,
    pub pending: String,
    pub ref_type: String,
    pub amount: f64,
    pub bill_date: String,
    pub ref_no: String,
}

#[derive(Debug, Serialize)]
pub struct ContactInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_person: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddressInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pincode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Thing>,
}

pub trait Doc {
    fn get_string(&self, key: &str) -> Option<String>;
    fn _get_document(&self, key: &str) -> Option<Document>;
    fn _get_f64(&self, key: &str) -> Option<f64>;
    fn get_array_document(&self, key: &str) -> Option<Vec<Document>>;
    fn get_array_thing(&self, key: &str, coll: &str) -> Option<HashSet<Thing>>;
    fn get_array_thing_from_str(&self, key: &str, coll: &str) -> Option<HashSet<Thing>>;
    fn get_oid_to_thing(&self, key: &str, coll: &str) -> Option<Thing>;
}

impl Doc for Document {
    fn get_string(&self, key: &str) -> Option<String> {
        self.get_str(key).map(|x| x.to_string()).ok()
    }
    fn _get_document(&self, key: &str) -> Option<Document> {
        self.get_document(key).ok().cloned()
    }
    fn get_array_document(&self, key: &str) -> Option<Vec<Document>> {
        self.get_array(key)
            .map(|x| {
                x.iter()
                    .map(|x| x.as_document().unwrap().clone())
                    .collect::<Vec<Document>>()
            })
            .ok()
    }

    fn _get_f64(&self, key: &str) -> Option<f64> {
        if let Ok(f) = self.get_f64(key) {
            return Some(f);
        } else if let Ok(i) = self.get_i64(key) {
            return Some(i as f64);
        } else if let Ok(i) = self.get_i32(key) {
            return Some(i as f64);
        }
        None
    }
    fn get_oid_to_thing(&self, key: &str, coll: &str) -> Option<Thing> {
        if let Ok(oid) = self.get_object_id(key) {
            return Some((coll.to_string(), oid.to_hex()).into());
        }
        None
    }
    fn get_array_thing(&self, key: &str, coll: &str) -> Option<HashSet<Thing>> {
        let mut res = HashSet::new();
        for item in self
            .get_array(key)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|x| x.as_object_id())
        {
            res.insert((coll.to_string(), item.to_hex()).into());
        }
        (!res.is_empty()).then_some(res)
    }

    fn get_array_thing_from_str(&self, key: &str, coll: &str) -> Option<HashSet<Thing>> {
        let mut res = HashSet::new();
        for item in self
            .get_array(key)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|x| x.as_str())
        {
            res.insert((coll.to_string(), item.to_string().to_lowercase()).into());
        }
        (!res.is_empty()).then_some(res)
    }
}

pub async fn duplicate_fix(db: &Database) {
    for collection in [
        "racks",
        "accounts",
        "inventories",
        "branches",
        "doctors",
        "pharma_salts",
        "units",
        "voucher_types",
        "sections",
        "manufacturers",
        "sale_incharges",
    ] {
        println!("{} duplicate fix start", collection);
        let docs = db
            .collection::<Document>(collection)
            .aggregate(
                vec![
                    doc! {"$group": {
                        "_id":"$validateName",
                        "ids": { "$addToSet": "$_id" }
                    }},
                    doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                    doc! { "$match": { "dup": true }},
                    doc! { "$project": { "ids": 1, "_id": 0 } },
                ],
                None,
            )
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let mut updates = Vec::new();
        for duplicates in docs {
            for (idx, dup_id) in duplicates
                .get_array("ids")
                .unwrap_or(&vec![])
                .iter()
                .map(|x| x.as_object_id().unwrap_or_default())
                .enumerate()
            {
                if idx != 0 {
                    updates.push(doc! {
                    "q": { "_id": dup_id },
                    "u": [{"$set": {
                        "name": {"$concat": ["$name", format!(" Dup{}", idx),]},
                        "displayName": {"$concat": ["$displayName", format!(" Dup{} ", idx),]}}}]
                    });
                }
            }
        }
        println!("count, {}", &updates.len());
        if !updates.is_empty() {
            let command = doc! {
                "update": collection,
                "updates": &updates
            };
            db.run_command(command, None).await.unwrap();
        }
        println!("{} duplicate fix end", collection);
    }
    println!("patients duplicate fix start");
    let docs = db
        .collection::<Document>("patients")
        .aggregate(
            vec![
                doc! {"$group": {
                    "_id": { "validateName": "$validateName", "customer": "$customer" },
                    "ids": { "$addToSet": "$_id" }
                }},
                doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                doc! { "$match": { "dup": true }},
                doc! { "$project": { "ids": 1, "_id": 0 } },
            ],
            None,
        )
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();
    let mut updates = Vec::new();
    for duplicates in docs {
        for (idx, dup_id) in duplicates
            .get_array("ids")
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_object_id().unwrap_or_default())
            .enumerate()
        {
            if idx != 0 {
                updates.push(doc! {"q": { "_id": dup_id }, "u": [{"$set": {"name": {"$concat": ["$name", format!(" Dup {}", idx),]}, "displayName": {"$concat": ["$displayName", format!(" Dup{} ", idx),]}} }]});
            }
        }
    }
    println!("count, {}", &updates.len());
    if !updates.is_empty() {
        let command = doc! {
            "update": "patients",
            "updates": &updates
        };
        db.run_command(command, None).await.unwrap();
    }
    println!("patients duplicate fix end");

    let docs = db
        .collection::<Document>("contacts")
        .aggregate(
            vec![
                doc! {"$group": {
                    "_id": { "validateName": "$validateName", "mob": "$contactInfo.mobile" },
                    "ids": { "$addToSet": "$_id" }
                }},
                doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                doc! { "$match": { "dup": true }},
                doc! { "$project": { "ids": 1, "_id": 0 } },
            ],
            None,
        )
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();
    let mut updates = Vec::new();
    for duplicates in docs {
        for (idx, dup_id) in duplicates
            .get_array("ids")
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_object_id().unwrap_or_default())
            .enumerate()
        {
            if idx != 0 {
                updates.push(doc! {"q": { "_id": dup_id }, "u": [{"$set": {"name": {"$concat": ["$name", format!("Dup{}", idx),]}, "displayName": {"$concat": ["$displayName", format!("Dup{} ", idx),]}} }]});
            }
        }
    }
    println!("count, {}", &updates.len());
    if !updates.is_empty() {
        let command = doc! {
            "update": "contacts",
            "updates": &updates
        };
        db.run_command(command, None).await.unwrap();
    }
    println!("contacts duplicate fix end");

    println!("batches duplicate fix start");
    let docs = db
        .collection::<Document>("batches")
        .aggregate(
            vec![
                doc! {"$group": {
                    "_id": { "batchNo": "$batchNo", "inventory": "$inventory", "branch": "$branch" },
                    "ids": { "$addToSet": "$_id" }
                }},
                doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                doc! { "$match": { "dup": true }},
                doc! { "$project": { "ids": 1, "_id": 0 } },
            ],
            None,
        )
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();
    let mut updates = Vec::new();
    for duplicates in docs {
        for (idx, dup_id) in duplicates
            .get_array("ids")
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_object_id().unwrap_or_default())
            .enumerate()
        {
            if idx != 0 {
                updates.push(doc! {"q": { "_id": dup_id }, "u": [{"$set": {"batchNo": {"$concat": ["$batchNo", format!("DUP{}", idx),]}} }]});
            }
        }
    }
    println!("count, {}", &updates.len());
    if !updates.is_empty() {
        let command = doc! {
            "update": "batches",
            "updates": &updates
        };
        db.run_command(command, None).await.unwrap();
    }
    println!("batches duplicate fix end");

    println!("member duplicate fix start");
    let docs = db
        .collection::<Document>("members")
        .aggregate(
            vec![
                doc! {"$group": {
                    "_id": "$username",
                    "ids": { "$addToSet": "$_id" }
                }},
                doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                doc! { "$match": { "dup": true }},
                doc! { "$project": { "ids": 1, "_id": 0 } },
            ],
            None,
        )
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();
    let mut updates = Vec::new();
    for duplicates in docs {
        for (idx, dup_id) in duplicates
            .get_array("ids")
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_object_id().unwrap_or_default())
            .enumerate()
        {
            if idx != 0 {
                updates.push(doc! {"q": { "_id": dup_id }, "u": [{"$set": {"username": {"$concat": ["$username", format!("dup{}", idx)]}} }]});
            }
        }
    }
    println!("count, {}", &updates.len());
    if !updates.is_empty() {
        let command = doc! {
            "update": "members",
            "updates": &updates
        };
        db.run_command(command, None).await.unwrap();
    }
    println!("members duplicate fix end");

    println!("print_templates duplicate fix start");
    let docs = db
        .collection::<Document>("print_templates")
        .aggregate(
            vec![
                doc! {"$group": {
                    "_id": "$name",
                    "ids": { "$addToSet": "$_id" }
                }},
                doc! { "$project": { "ids": 1, "dup": { "$gt": [{ "$size": "$ids" }, 1] } } },
                doc! { "$match": { "dup": true }},
                doc! { "$project": { "ids": 1, "_id": 0 } },
            ],
            None,
        )
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();
    let mut updates = Vec::new();
    for duplicates in docs {
        for (idx, dup_id) in duplicates
            .get_array("ids")
            .unwrap_or(&vec![])
            .iter()
            .map(|x| x.as_object_id().unwrap_or_default())
            .enumerate()
        {
            if idx != 0 {
                updates.push(doc! {"q": { "_id": dup_id }, "u": [{"$set": {"name": {"$concat": ["$name", format!("dup{}", idx)]}} }]});
            }
        }
    }
    println!("count, {}", &updates.len());
    if !updates.is_empty() {
        let command = doc! {
            "update": "print_templates",
            "updates": &updates
        };
        db.run_command(command, None).await.unwrap();
    }
    db
    .collection::<Document>("voucher_types")
    .update_many(
        doc!{"voucherType": {"$in": ["MANUFACTURING_JOURNAL", "MATERIAL_CONVERSION", "STOCK_TRANSFER"]}}, 
        vec![doc! {"$set": {"default": false,"voucherType": "STOCK_ADJUSTMENT", "name": {"$concat": ["$name", " StkAdj"]}, "displayName": {"$concat": ["$displayName", " StkAdj"]}}}], 
        None
    )
    .await.unwrap();
    println!("print_templates duplicate fix end");
}
