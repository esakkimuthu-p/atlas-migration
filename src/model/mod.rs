use std::collections::HashSet;

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Database,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client as SurrealClient, sql::Thing, Surreal};

mod account;
mod account_transaction;
mod batch;
mod desktop_client;
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

pub trait Doc {
    fn get_string(&self, key: &str) -> Option<String>;
    fn _get_document(&self, key: &str) -> Option<Document>;
    fn _get_f64(&self, key: &str) -> Option<f64>;
    // fn get_oid_hex(&self, key: &str) -> Option<String>;
    fn get_array_document(&self, key: &str) -> Option<Vec<Document>>;
    fn get_array_thing(&self, key: &str, coll: &str) -> Option<HashSet<Thing>>;
    fn get_oid_to_thing(&self, key: &str, coll: &str) -> Option<Thing>;
    fn get_chrono_datetime(&self, key: &str) -> Option<DateTime<Utc>>;
}

impl Doc for Document {
    fn get_string(&self, key: &str) -> Option<String> {
        self.get_str(key).map(|x| x.to_string()).ok()
    }
    fn get_chrono_datetime(&self, key: &str) -> Option<DateTime<Utc>> {
        self.get_datetime(key).map(|x| x.to_chrono()).ok()
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
    // fn get_oid_hex(&self, key: &str) -> Option<String> {
    //     self.get_object_id(key).map(|x| x.to_hex()).ok()
    // }
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
