use std::collections::HashSet;

use futures_util::StreamExt;
use mongodb::{
    bson::{doc, from_document, DateTime, Document},
    Client as MongoClient,
};
use serde::Deserialize;

use surrealdb::{
    engine::remote::ws::Client as SurrealClient, engine::remote::ws::Ws, opt::auth::Root,
    sql::Thing, Surreal,
};

mod model;

use model::{
    AccountTransaction, Batch, Inventory, InventoryCess, InventoryTransaction, InventoryUnit,
};

#[derive(Deserialize, Clone)]
struct Created {
    id: Thing,
}

pub trait Doc {
    fn get_string(&self, key: &str) -> Option<String>;
    fn _get_document(&self, key: &str) -> Option<Document>;
    fn _get_f64(&self, key: &str) -> Option<f64>;
    fn get_oid_hex(&self, key: &str) -> Option<String>;
    fn get_array_document(&self, key: &str) -> Option<Vec<Document>>;
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
            .and_then(|x| {
                Ok(x.iter()
                    .map(|x| x.as_document().unwrap().clone())
                    .collect::<Vec<Document>>())
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
    fn get_oid_hex(&self, key: &str) -> Option<String> {
        self.get_object_id(key).map(|x| x.to_hex()).ok()
    }
}

pub static DB: Surreal<SurrealClient> = Surreal::init();

#[tokio::main]
async fn main() {
    DB.connect::<Ws>("localhost:8000")
        .await
        .expect("Error connecting to database");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    DB.use_ns("test").await.unwrap();
    DB.use_db("test").await.unwrap();

    let db = MongoClient::with_uri_str(
        "mongodb+srv://testadmin:rootroot@auditplus-test.dqqxs.mongodb.net/velavanmed",
    )
    .await
    .unwrap()
    .default_database()
    .unwrap();
    println!("{:?}", db.name());
    // inventory_transactions
    /*
    println!("inventory_transaction INDEX start");
    DB.query("DEFINE INDEX inv ON TABLE inventory_transaction COLUMNS inventory")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX man ON TABLE inventory_transaction COLUMNS manufacturer")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX sec ON TABLE inventory_transaction COLUMNS section")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX date ON TABLE inventory_transaction COLUMNS date")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();

    DB.query("DEFINE INDEX branch ON TABLE inventory_transaction COLUMNS branch")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query(
        "DEFINE INDEX voucher_type_base ON TABLE inventory_transaction COLUMNS voucher_type_base",
    )
    .await
    .unwrap()
    .take::<Option<()>>(0)
    .unwrap();
    println!("inventory_transaction INDEX end");
    println!("inventory_transaction download start");
    let mut cur = db
        .collection::<Document>("inventory_transactions")
        .find(doc! {}, None)
        .await
        .unwrap();
    while let Some(Ok(d)) = cur.next().await {
        let _created: Created = DB
            .create("inventory_transaction")
            .content(InventoryTransaction {
                date: d.get_string("date").unwrap(),
                inward: d._get_f64("inward").unwrap(),
                outward: d._get_f64("outward").unwrap(),
                inventory: ("inventory".to_string(), d.get_oid_hex("inventory").unwrap()).into(),
                inventory_name: d.get_string("inventoryName").unwrap(),
                manufacturer: d
                    .get_oid_hex("manufacturerId")
                    .map(|x| ("manufacturer".to_string(), x).into()),
                manufacturer_name: d.get_string("manufacturerName"),
                act: d.get_bool("act").unwrap_or_default(),
                act_hide: d.get_bool("actHide").unwrap_or_default(),
                batch: ("batch".to_string(), d.get_oid_hex("adjBatch").unwrap()).into(),
                branch: ("branch".to_string(), d.get_oid_hex("branch").unwrap()).into(),
                branch_name: d.get_string("branchName").unwrap(),
                unit_conv: d._get_f64("unitConv").unwrap(),
                unit: ("unit".to_string(), d.get_oid_hex("unitId").unwrap()).into(),
                unit_name: d.get_string("unitName").unwrap(),
                ref_no: d.get_string("refNo"),
                voucher_no: d.get_string("voucherNo"),
                voucher_type_base: d.get_string("voucherType"),
                voucher_type: d
                    .get_oid_hex("voucherTypeId")
                    .map(|x| ("voucher_type".to_string(), x).into()),
                voucher: d
                    .get_oid_hex("voucherId")
                    .map(|x| ("voucher".to_string(), x).into()),
                section: d
                    .get_oid_hex("sectionId")
                    .map(|x| ("section".to_string(), x).into()),
                section_name: d.get_string("sectionName"),
                contact: d
                    .get_oid_hex("customerId")
                    .or(d.get_oid_hex("vendorId"))
                    .map(|x| ("contact".to_string(), x).into()),
                contact_name: d.get_string("customerName").or(d.get_string("vendorName")),
                alt_account: d
                    .get_oid_hex("altAccount")
                    .map(|x| ("account".to_string(), x).into()),
                alt_account_name: d.get_string("altAccountName"),
                asset_amount: d._get_f64("assetAmount"),
                taxable_amount: d._get_f64("taxableAmount"),
                cgst_amount: d._get_f64("cgstAmount"),
                cess_amount: d._get_f64("cessAmount"),
                sgst_amount: d._get_f64("sgstAmount"),
                igst_amount: d._get_f64("igstAmount"),
                nlc: d._get_f64("nlc"),
                is_opening: d.get_bool("isOpening").ok(),
            })
            .await
            .unwrap()
            .first()
            .cloned()
            .unwrap();
    }
    println!("inventory_transaction download end");
    // account_transactions
    println!("account_transaction INDEX start");
    DB.query("DEFINE INDEX acc ON TABLE account_transaction COLUMNS account")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX acc_type ON TABLE account_transaction COLUMNS account_type")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX voucher_id ON TABLE account_transaction COLUMNS voucher")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX date ON TABLE account_transaction COLUMNS date")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();

    DB.query("DEFINE INDEX branch ON TABLE account_transaction COLUMNS branch")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query(
        "DEFINE INDEX voucher_type_base ON TABLE account_transaction COLUMNS voucher_type_base",
    )
    .await
    .unwrap()
    .take::<Option<()>>(0)
    .unwrap();
    println!("account_transaction INDEX end");
    println!("account_transaction download start");
    let mut cur = db
        .collection::<Document>("account_transactions")
        .find(doc! {}, None)
        .await
        .unwrap();
    while let Some(Ok(d)) = cur.next().await {
        let _created: Created = DB
            .create("account_transaction")
            .content(AccountTransaction {
                date: d.get_string("date").unwrap(),
                debit: d._get_f64("debit").unwrap(),
                credit: d._get_f64("credit").unwrap(),
                account: ("account".to_string(), d.get_oid_hex("account").unwrap()).into(),
                account_name: d.get_string("accountName").unwrap(),
                account_type: d.get_string("accountType").unwrap(),
                branch: ("branch".to_string(), d.get_oid_hex("branch").unwrap()).into(),
                branch_name: d.get_string("branchName").unwrap(),
                act: d.get_bool("act").unwrap_or_default(),
                act_hide: d.get_bool("actHide").unwrap_or_default(),
                alt_account: d
                    .get_oid_hex("altAccount")
                    .map(|x| ("account".to_string(), x).into()),
                alt_account_name: d.get_string("altAccountName"),
                ref_no: d.get_string("refNo"),
                voucher_no: d.get_string("voucherNo"),
                voucher_type_base: d.get_string("voucherType"),
                voucher_type: d
                    .get_oid_hex("voucherTypeId")
                    .map(|x| ("voucher_type".to_string(), x).into()),
                voucher: d
                    .get_oid_hex("voucherId")
                    .map(|x| ("voucher".to_string(), x).into()),
                is_opening: d.get_bool("isOpening").ok(),
                eff_date: d.get_string("effDate"),
                voucher_mode: d.get_string("voucherMode"),
            })
            .await
            .unwrap()
            .first()
            .cloned()
            .unwrap();
    }
    println!("account_transaction download end");
    println!("inventory INDEX start");
    DB.query("DEFINE INDEX validate_name ON TABLE inventory COLUMNS validate_name")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX barcodes ON TABLE inventory COLUMNS barcodes")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    println!("inventory INDEX end");
    // inventories
    println!("inventory download start");
    let mut cur = db
        .collection::<Document>("inventories")
        .find(doc! {}, None)
        .await
        .unwrap();
    while let Some(Ok(d)) = cur.next().await {
        let cess = d
            .get_document("cess")
            .ok()
            .and_then(|x| from_document::<InventoryCess>(x.clone()).ok());
        let mut units = Vec::new();
        for u_item in d.get_array_document("units").unwrap_or_default() {
            let unit = InventoryUnit {
                unit: ("unit".to_string(), u_item.get_oid_hex("unitId").unwrap()).into(),
                unit_name: u_item.get_string("unitName").unwrap(),
                conversion: u_item._get_f64("conversion").unwrap(),
                preferred_for_purchase: u_item.get_bool("preferredForPurchase").unwrap_or_default(),
                preferred_for_sale: u_item.get_bool("preferredForPurchase").unwrap_or_default(),
            };
            units.push(unit);
        }

        let barcodes = d
            .get_array("barcodes")
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|x| {
                x.as_str()
                    .is_some()
                    .then_some(x.as_str().unwrap_or_default().to_string())
            })
            .collect::<HashSet<String>>();
        let _created: Created = DB
            .create("inventory")
            .content(Inventory {
                id: ("inventory".to_string(), d.get_oid_hex("_id").unwrap()).into(),
                name: d.get_string("name").unwrap(),
                validate_name: d.get_string("validateName").unwrap(),
                display_name: d.get_string("displayName").unwrap(),
                precision: d._get_f64("precision").unwrap() as u8,
                head: ("inventory_head".to_string(), d.get_oid_hex("head").unwrap()).into(),
                allow_negative_stock: d.get_bool("allowNegativeStock").unwrap_or_default(),
                tax: ("tax".to_string(), d.get_string("tax").unwrap()).into(), // gst0.1....
                units,
                cess,
                barcodes: (!barcodes.is_empty()).then_some(barcodes),
                hsn_code: d.get_string("hsnCode"),
                description: d.get_string("description"),
                section: d
                    .get_oid_hex("sectionId")
                    .map(|x| ("section".to_string(), x).into()),
                section_name: d.get_string("sectionName"),
                manufacturer: d
                    .get_oid_hex("manufacturerId")
                    .map(|x| ("manufacturer".to_string(), x).into()),
                manufacturer_name: d.get_string("manufacturerName"),
                vendor: d
                    .get_oid_hex("vendorId")
                    .map(|x| ("contact".to_string(), x).into()),
                vendor_name: d.get_string("vendorName"),
                vendors: None,
                salts: None,
                schedule_h: d.get_bool("scheduleH").ok(),
                schedule_h1: d.get_bool("scheduleH1").ok(),
                narcotics: d.get_bool("narcotics").ok(),
                enable_expiry: d.get_bool("enableExpiry").unwrap_or_default(),
            })
            .await
            .unwrap()
            .first()
            .cloned()
            .unwrap();
    }
    println!("inventory download end");
    */
    // batches
    println!("batch INDEX start");
    DB.query("DEFINE INDEX branch ON TABLE batch COLUMNS branch")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX barcode ON TABLE batch COLUMNS barcode")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX inv ON TABLE batch COLUMNS inventory")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX sec ON TABLE batch COLUMNS section")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    DB.query("DEFINE INDEX man ON TABLE batch COLUMNS manufacturer")
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
    println!("batch INDEX end");
    println!("batch download start");
    let mut cur = db
        .collection::<Document>("batches")
        .find(doc! {}, None)
        .await
        .unwrap();
    while let Some(Ok(d)) = cur.next().await {
        let _created: Created = DB
            .create("batch")
            .content(Batch {
                id: ("batch".to_string(), d.get_oid_hex("_id").unwrap()).into(),
                barcode: ("barcode".to_string(), d.get_oid_hex("barcode").unwrap()).into(),
                batch_no: d.get_string("batchNo"),
                last_inward: d.get_string("lastInward").unwrap_or_default(),
                inward: d._get_f64("inward").unwrap(),
                outward: d._get_f64("outward").unwrap(),
                inventory: ("inventory".to_string(), d.get_oid_hex("inventory").unwrap()).into(),
                inventory_name: d.get_string("inventoryName").unwrap(),
                expiry: d.get_string("expiry"),
                manufacturer: d
                    .get_oid_hex("manufacturerId")
                    .map(|x| ("manufacturer".to_string(), x).into()),
                manufacturer_name: d.get_string("manufacturerName"),
                branch: ("branch".to_string(), d.get_oid_hex("branch").unwrap()).into(),
                branch_name: d.get_string("branchName").unwrap(),
                unit_conv: d._get_f64("unitConv").unwrap(),
                unit: ("unit".to_string(), d.get_oid_hex("unitId").unwrap()).into(),
                unit_name: d.get_string("unitName").unwrap(),
                section: d
                    .get_oid_hex("sectionId")
                    .map(|x| ("section".to_string(), x).into()),
                section_name: d.get_string("sectionName"),
                mrp: d._get_f64("mrp"),
                s_rate: d._get_f64("sRate"),
                p_rate: d._get_f64("pRate"),
                avg_nlc: d._get_f64("avgNlc"),
                first_nlc: d._get_f64("firstNlc"),
                last_nlc: d._get_f64("lastNlc"),
                max_nlc: d._get_f64("maxNlc"),
                min_nlc: d._get_f64("minNlc"),
                p_rate_tax_inc: d.get_bool("pRateTaxInc").unwrap_or_default(),
                s_rate_tax_inc: d.get_bool("sRateTaxInc").unwrap_or(true),
                vendors: None,
                // created:
            })
            .await
            .unwrap()
            .first()
            .cloned()
            .unwrap();
    }
    println!("batch download end");
}
