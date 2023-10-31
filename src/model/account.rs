use super::{
    doc, serialize_opt_tax_as_thing, Created, Database, Doc, Document, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};
use mongodb::options::FindOptions;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_favour_of: Option<String>,
    pub account_type: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sac_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tds_nature_of_payment: Option<Thing>,
}

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

    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>("accounts")
            .find(doc! {"surrealId": {"$exists": false}}, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("account")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "account").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    alias_name: d.get_string("aliasName"),
                    in_favour_of: d.get_string("inFavourOf"),
                    account_type: (
                        "account_type".to_string(),
                        d.get_string("accountType").unwrap().to_lowercase(),
                    )
                        .into(),
                    gst_tax: d.get_string("tax"),
                    gst_type: d.get_string("gstType"),
                    sac_code: d.get_string("sacCode"),
                    parent: d.get_oid_to_thing("parentAccount", "account"),
                    description: d.get_string("description"),
                    tds_nature_of_payment: d
                        .get_oid_to_thing("tdsNatureOfPayment", "tds_nature_of_payment"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("account download end");
    }
}
