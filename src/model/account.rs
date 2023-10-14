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
    pub is_default: Option<bool>,
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
    pub hide: bool,
}

impl Account {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("account download start");
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>("accounts")
            .find(doc! {}, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let mut id = d.get_oid_to_thing("_id", "account").unwrap();
            let mut is_default = None;
            if let Some(default_acc) = d.get_string("defaultName") {
                id = ("account".to_string(), default_acc.to_lowercase()).into();
                is_default = Some(true);
            }
            let _created: Created = surrealdb
                .create("account")
                .content(Self {
                    id,
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    is_default,
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
                    hide: d.get_bool("hide").unwrap_or_default(),
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
