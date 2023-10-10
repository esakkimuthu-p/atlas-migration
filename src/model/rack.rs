use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct Rack {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
}

impl Rack {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("rack INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE rack COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("rack INDEX end");
        println!("rack download start");
        let mut cur = mongodb
            .collection::<Document>("racks")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("rack")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "rack").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("rack download end");
    }
}
