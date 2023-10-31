use std::str::FromStr;

use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
use mongodb::{bson::oid::ObjectId, options::FindOptions};

#[derive(Debug, Serialize)]
pub struct Batch {
    pub id: Thing,
    pub inventory: Thing,
    pub branch: Thing,
    pub barcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    pub last_inward: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_rate: Option<f64>,
    pub p_rate_tax_inc: bool,
    pub s_rate_tax_inc: bool,
    pub unit_conv: f64,
}

impl Batch {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        mongodb
            .collection::<Document>("batches")
            .delete_one(
                doc! {"_id": ObjectId::from_str("647c6146a93905cdc7396905").unwrap()},
                None,
            )
            .await
            .unwrap();
        let mut cur = mongodb
            .collection::<Document>("batches")
            .find(doc! {}, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("batch")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "batch").unwrap(),
                    barcode: d.get_object_id("barcode").unwrap().to_hex(),
                    batch_no: d.get_string("batchNo"),
                    last_inward: d.get_string("lastInward").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    expiry: d.get_string("expiry"),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap(),
                    mrp: d._get_f64("mrp"),
                    s_rate: d._get_f64("sRate"),
                    p_rate_tax_inc: d.get_bool("pRateTaxInc").unwrap_or_default(),
                    s_rate_tax_inc: d.get_bool("sRateTaxInc").unwrap_or(true),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("batch download end");
    }
}
