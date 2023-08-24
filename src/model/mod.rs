use std::collections::HashSet;

use futures_util::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    Database,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client as SurrealClient, sql::Datetime, sql::Thing, Surreal};

mod account;
mod account_opening;
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
mod inventory;
mod inventory_head;
mod inventory_opening;
mod inventory_transaction;
mod manufacturer;
mod member;
mod pharma_salt;
mod rack;
mod section;
mod unit;
mod voucher_numbering;

pub use account::Account;
pub use account_opening::AccountOpening;
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
pub use inventory::*;
pub use inventory_head::InventoryHead;
pub use inventory_opening::InventoryOpening;
pub use inventory_transaction::InventoryTransaction;
pub use manufacturer::Manufacturer;
pub use member::Member;
pub use pharma_salt::PharmaSalt;
pub use rack::Rack;
pub use section::Section;
pub use unit::Unit;
pub use voucher_numbering::VoucherNumbering;

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
            res.insert((coll.to_string(), item.to_string()).into());
        }
        (!res.is_empty()).then_some(res)
    }
}
