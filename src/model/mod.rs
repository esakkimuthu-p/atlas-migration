use std::collections::HashSet;

use futures_util::StreamExt;
use mongodb::{
    bson::{doc, to_bson, Bson, Document},
    Database,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client as SurrealClient, sql::Datetime, sql::Thing, Surreal};

mod account;
mod account_transaction;
mod batch;
mod desktop_client;
mod discount_code;
mod doctor;
mod inventory;
mod inventory_transaction;
mod pharma_salt;
mod rack;
mod section;
mod unit;

pub use account::Account;
pub use account_transaction::AccountTransaction;
pub use batch::Batch;
pub use desktop_client::DesktopClient;
pub use discount_code::DiscountCode;
pub use doctor::Doctor;
pub use inventory::*;
pub use inventory_transaction::InventoryTransaction;
pub use pharma_salt::PharmaSalt;
pub use rack::Rack;
pub use section::Section;
pub use unit::Unit;

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

pub trait Doc {
    fn get_string(&self, key: &str) -> Option<String>;
    fn _get_document(&self, key: &str) -> Option<Document>;
    fn _get_f64(&self, key: &str) -> Option<f64>;
    fn get_array_document(&self, key: &str) -> Option<Vec<Document>>;
    fn get_array_thing(&self, key: &str, coll: &str) -> Option<HashSet<Thing>>;
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
}
