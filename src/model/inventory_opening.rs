use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};

#[derive(Debug, Serialize)]
pub struct InventoryOpening {
    pub inventory: Thing,
    pub branch: Thing,
    pub act_hide: bool,
    pub act: bool,
    pub updated_at: Datetime,
}

impl InventoryOpening {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("inventory_opening INDEX start");
        surrealdb
            .query("DEFINE INDEX br ON TABLE inventory_opening COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX inv ON TABLE inventory_opening COLUMNS inventory")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("inventory_opening INDEX end");
        println!("inventory_opening download start");
        let mut cur = mongodb
            .collection::<Document>("inventory_openings")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("inventory_opening")
                .content(Self {
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    act: d.get_bool("act").unwrap_or_default(),
                    updated_at: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("inventory_opening download end");
    }
}
