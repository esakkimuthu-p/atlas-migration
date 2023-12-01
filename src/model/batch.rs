use std::str::FromStr;

use super::{doc, Database, Doc, Document, PostgresClient, StreamExt};
use mongodb::{bson::oid::ObjectId, options::FindOptions};

pub struct Batch;

impl Batch {
    pub async fn create(postgres: &PostgresClient, mongodb: &Database) {
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
            postgres
                .execute(
                    "INSERT INTO batch (id,barcode,batch_no,inventory,expiry,branch,unit_conv,mrp,s_rate,p_rate_tax_inc,s_rate_tax_inc) 
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                    &[
                        &d.get_object_id("_id").unwrap().to_hex(),
                        &d.get_object_id("barcode").unwrap().to_hex(),
                        &d.get_string("batchNo"),
                        &d.get_object_id("inventory").unwrap().to_hex(),
                        &d.get_string("expiry"),
                        &d.get_object_id("branch").unwrap().to_hex(),
                        &d._get_f64("unitConv").unwrap(),
                        &d._get_f64("mrp"),
                        &d._get_f64("sRate"),
                        &d.get_bool("pRateTaxInc").unwrap_or_default(),
                        &d.get_bool("sRateTaxInc").unwrap_or(true),
                    ],
                )
                .await
                .unwrap();
        }
        println!("batch download end");
    }
}
