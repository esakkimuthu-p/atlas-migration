use super::{doc, Database, Doc, Document, PostgresClient, StreamExt};
use mongodb::options::FindOptions;

pub struct Account;

impl Account {
    pub async fn map(mongodb: &Database) {
        let updates = vec![
            doc! {"q": { "defaultName" : {"$regex": "SALE"}}, "u": {"$set": {"surrealId": "sales"}}, "multi": true},
            doc! {"q": { "defaultName" : {"$regex":"PURCHASE"}}, "u": {"$set": {"surrealId": "purchases"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"SGST_PAYABLE"}}, "u": {"$set": {"surrealId": "sgst_payable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"CGST_PAYABLE"}}, "u": {"$set": {"surrealId": "cgst_payable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"IGST_PAYABLE"}}, "u": {"$set": {"surrealId": "igst_payable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"CESS_PAYABLE"}}, "u": {"$set": {"surrealId": "cess_payable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"SGST_RECEIVABLE"}}, "u": {"$set": {"surrealId": "sgst_receivable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"CGST_RECEIVABLE"}}, "u": {"$set": {"surrealId": "cgst_receivable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"IGST_RECEIVABLE"}}, "u": {"$set": {"surrealId": "igst_receivable"} },"multi": true},
            doc! {"q": { "defaultName" : {"$regex":"IGST_RECEIVABLE"}}, "u": {"$set": {"surrealId": "cess_receivable"} },"multi": true},
            doc! {"q": { "defaultName": "CASH"}, "u": {"$set": {"surrealId": "cash"} }},
        ];
        let command = doc! {
            "update": "accounts",
            "updates": &updates
        };
        mongodb.run_command(command, None).await.unwrap();
    }

    pub async fn create(postgres: &PostgresClient, mongodb: &Database) {
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>("accounts")
            .find(doc! {"surrealId": {"$exists": false}}, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            postgres
                .execute(
                    "INSERT INTO account (id,name,display_name,alias_name,in_favour_of,account_type,gst_tax,gst_type,sac_code,parent,description,tds_nature_of_payment) 
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                    &[
                        &d.get_object_id("_id").unwrap().to_hex(),
                        &d.get_string("name").unwrap(),
                        &d.get_string("displayName").unwrap(),
                        &d.get_string("aliasName"),
                        &d.get_string("inFavourOf"),
                        &d.get_string("accountType").unwrap().to_lowercase(),
                        &d.get_string("tax"),
                        &d.get_string("gstType"),
                        &d.get_string("sacCode"),
                        &d.get_object_id("parentAccount").ok().map(|x|x.to_hex()),
                        &d.get_string("description"),
                        &d.get_object_id("tdsNatureOfPayment").ok().map(|x|x.to_hex())
                    ],
                )
                .await
                .unwrap();
        }
        println!("account download end");
    }
}
