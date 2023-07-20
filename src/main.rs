use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Client as MongoClient,
};
use serde::Deserialize;

use surrealdb::{
    engine::remote::ws::Client as SurrealClient, engine::remote::ws::Ws, opt::auth::Root,
    sql::Thing, Surreal,
};

mod acc_trn;
mod inv_trn;

use acc_trn::AccountTransaction;
use inv_trn::InventoryTransaction;

#[derive(Deserialize, Clone)]
struct Created {
    id: Thing,
}

pub trait Doc {
    fn get_string(&self, key: &str) -> Option<String>;
    fn _get_f64(&self, key: &str) -> Option<f64>;
    fn get_oid_hex(&self, key: &str) -> Option<String>;
}

impl Doc for Document {
    fn get_string(&self, key: &str) -> Option<String> {
        self.get_str(key).map(|x| x.to_string()).ok()
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
    DB.use_db("velavanmedical").await.unwrap();

    let db = MongoClient::with_uri_str("mongodb://localhost:27017/velavanmedical")
        .await
        .unwrap()
        .default_database()
        .unwrap();
    println!("{:?}", db.name());

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
}
