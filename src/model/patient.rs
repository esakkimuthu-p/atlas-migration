use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct Patient {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub customer: Thing,
}

impl Patient {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("patients")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("patient")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "patient").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    customer: d.get_oid_to_thing("customer", "contact").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("patient download end");
    }
}
