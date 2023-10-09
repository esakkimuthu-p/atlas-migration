use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct Unit {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub uqc: Thing,
    pub symbol: String,
}

impl Unit {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("unit INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE unit COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("unit INDEX end");
        println!("unit download start");
        let mut cur = mongodb
            .collection::<Document>("units")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("unit")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "unit").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    symbol: d.get_string("symbol").unwrap(),
                    uqc: (
                        "uqc".to_string(),
                        d.get_string("uqc").unwrap().to_lowercase(),
                    )
                        .into(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("unit download end");
    }
}
