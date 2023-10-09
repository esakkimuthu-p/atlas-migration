use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use futures_util::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    Database,
};
use serde::{Deserialize, Serialize, Serializer};
use surrealdb::{
    engine::remote::ws::Client as SurrealClient,
    sql::Datetime,
    sql::{Id, Thing},
    Surreal,
};

mod account;
// mod account_opening;
mod account_transaction;
mod bank_txn;
mod batch;
mod bill_allocation;
mod branch;
mod cash_register;
mod contact;
mod desktop_client;
mod discount_code;
mod doctor;
mod financial_year;
mod gst_registration;
mod inventory;
mod inventory_head;
mod inventory_opening;
mod inventory_transaction;
mod manufacturer;
mod member;
mod patient;
mod pharma_salt;
mod pos_terminal;
mod print_template;
mod rack;
mod sale_incharge;
mod section;
mod tds_nature_of_payment;
mod unit;
mod vendor_inventory_map;
mod voucher;
mod voucher_numbering;
mod voucher_type;

pub use account::Account;
// pub use account_opening::AccountOpening;
pub use account_transaction::AccountTransaction;
pub use bank_txn::BankTransaction;
pub use batch::Batch;
pub use bill_allocation::BillAllocation;
pub use branch::Branch;
pub use cash_register::CashRegister;
pub use contact::Contact;
pub use desktop_client::DesktopClient;
pub use discount_code::DiscountCode;
pub use doctor::Doctor;
pub use financial_year::FinancialYear;
pub use gst_registration::GstRegistration;
pub use inventory::*;
pub use inventory_head::InventoryHead;
pub use inventory_opening::InventoryOpening;
pub use inventory_transaction::InventoryTransaction;
pub use manufacturer::Manufacturer;
pub use member::Member;
pub use patient::Patient;
pub use pharma_salt::PharmaSalt;
pub use pos_terminal::PosTerminal;
pub use print_template::PrintTemplate;
pub use rack::Rack;
pub use sale_incharge::SaleIncharge;
pub use section::Section;
pub use tds_nature_of_payment::TdsNatureOfPayment;
pub use unit::Unit;
pub use vendor_inventory_map::{VendorBillMap, VendorItemMap};
pub use voucher::Voucher;
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

fn serialize_tax_as_thing<S: Serializer>(val: &String, ser: S) -> Result<S::Ok, S::Error> {
    let tax_id = match val.as_str() {
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

fn serialize_round_2<S: Serializer>(val: &f64, ser: S) -> Result<S::Ok, S::Error> {
    let x = (val * 10_f64.powi(2)).round() / 10_f64.powi(2);
    x.serialize(ser)
}

fn serialize_round_4<S: Serializer>(val: &f64, ser: S) -> Result<S::Ok, S::Error> {
    let x = (val * 10_f64.powi(4)).round() / 10_f64.powi(4);
    x.serialize(ser)
}

fn serialize_opt_round_2<S: Serializer>(val: &Option<f64>, ser: S) -> Result<S::Ok, S::Error> {
    match val {
        Some(num) => ((num * 10_f64.powi(2)).round() / 10_f64.powi(2)).serialize(ser),
        _ => unreachable!(),
    }
}

fn serialize_opt_round_4<S: Serializer>(val: &Option<f64>, ser: S) -> Result<S::Ok, S::Error> {
    match val {
        Some(num) => ((num * 10_f64.powi(4)).round() / 10_f64.powi(4)).serialize(ser),
        _ => unreachable!(),
    }
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
    fn get_surreal_datetime(&self, key: &str) -> Option<Datetime>;
    fn get_surreal_datetime_from_str(&self, key: &str) -> Option<Datetime>;
}

impl Doc for Document {
    fn get_string(&self, key: &str) -> Option<String> {
        self.get_str(key).map(|x| x.to_string()).ok()
    }
    fn get_surreal_datetime(&self, key: &str) -> Option<Datetime> {
        self.get_datetime(key).ok().map(|x| x.to_chrono().into())
    }
    fn get_surreal_datetime_from_str(&self, key: &str) -> Option<Datetime> {
        self.get_str(key)
            .ok()
            .map(|x| Datetime::try_from(x).unwrap())
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
