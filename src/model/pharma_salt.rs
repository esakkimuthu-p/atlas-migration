use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct PharmaSalt {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
}

impl PharmaSalt {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("pharma_salt INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE pharma_salt COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("pharma_salt INDEX end");
        println!("pharma_salt download start");
        let mut cur = mongodb
            .collection::<Document>("pharma_salts")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("pharma_salt")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "pharma_salt").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("pharma_salt download end");
    }
}
