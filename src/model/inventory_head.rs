use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct InventoryHead {
    pub id: Thing,
    pub name: String,
    pub created: Datetime,
    pub updated: Datetime,
}

impl InventoryHead {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("inventory_head INDEX start");
        surrealdb
            .query("DEFINE INDEX name ON TABLE inventory_head COLUMNS name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("inventory_head INDEX end");
        println!("inventory_head download start");
        let mut cur = mongodb
            .collection::<Document>("inventory_heads")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let mut id = d.get_oid_to_thing("_id", "inventory_head").unwrap();
            if d.get_string("defaultName").is_some()
                || d.get_string("name").unwrap_or_default().to_lowercase() == "default"
            {
                id = ("inventory_head".to_string(), "default".to_string()).into();
            }
            let _created: Created = surrealdb
                .create("inventory_head")
                .content(Self {
                    id,
                    name: d.get_string("name").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("inventory_head download end");
    }
}
