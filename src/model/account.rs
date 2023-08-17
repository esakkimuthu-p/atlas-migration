use super::{
    doc, Created, Database, DateTime, Doc, Document, HashSet, Serialize, StreamExt, Surreal,
    SurrealClient, Thing, Utc,
};

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: Thing,
    pub name: String,
    pub val_name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_favour_of: Option<String>,
    pub account_type: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sac_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<HashSet<Thing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tds_nature_of_payment: Option<Thing>,
    pub hide: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Account {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("account INDEX start");
        surrealdb
            .query("DEFINE INDEX account_type ON TABLE account COLUMNS account_type")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE account COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("account INDEX end");
        println!("account download start");
        let mut cur = mongodb
            .collection::<Document>("accounts")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let mut id = d.get_oid_to_thing("_id", "account").unwrap();
            let mut is_default = None;
            if let Ok(default_acc) = d.get_str("defaultName") {
                id = (
                    "account".to_string(),
                    default_acc.to_string().to_lowercase(),
                )
                    .into();
                is_default = Some(true);
            }
            let _created: Created = surrealdb
                .create("account")
                .content(Self {
                    id,
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    is_default,
                    alias_name: d.get_string("aliasName"),
                    val_alias_name: d.get_string("validateAliasName"),
                    in_favour_of: d.get_string("inFavourOf"),
                    account_type: (
                        "account_type".to_string(),
                        d.get_string("accountType")
                            .unwrap()
                            .to_string()
                            .to_lowercase(),
                    )
                        .into(),
                    tax: d.get_string("tax").map(|x| ("tax".to_string(), x).into()),
                    gst_type: d.get_string("gstType"),
                    sac_code: d.get_string("sacCode"),
                    parent_account: d.get_oid_to_thing("parentAccount", "account"),
                    parents: d.get_array_thing("parents", "account"),
                    description: d.get_string("description"),
                    tds_nature_of_payment: d
                        .get_oid_to_thing("tdsNatureOfPayment", "tds_nature_of_payment"),
                    hide: d.get_bool("hide").unwrap_or_default(),
                    created: d.get_chrono_datetime("createdAt").unwrap_or_default(),
                    updated: d.get_chrono_datetime("updatedAt").unwrap_or_default(),
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
