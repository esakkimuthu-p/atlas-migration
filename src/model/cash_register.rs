use std::collections::HashSet;

use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct CashRegister {
    pub id: Thing,
    pub name: String,
    pub val_name: String,
    pub display_name: String,
    pub branches: Option<HashSet<Thing>>,
    pub controller: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening: Option<f64>,
}
impl CashRegister {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("cash_register INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE cash_register COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("cash_register INDEX end");
        println!("cash_register download start");
        let mut cur = mongodb
            .collection::<Document>("cash_registers")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("cash_register")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "cash_register").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                    branches: d.get_array_thing("branches", "branch"),
                    controller: d.get_oid_to_thing("controller", "member").unwrap(),
                    controller_only: d.get_bool("controllerOnly").ok(),
                    opening: d._get_f64("opening"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("cash_register download end");
    }
}
