use super::{
    doc, Created, Database, DateTime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing, Utc,
};
#[derive(Debug, Serialize)]
pub struct Unit {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    pub uqc: Thing,
    pub symbol: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Unit {
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
                    id: d.get_oid_to_thing("_id", "unit").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    symbol: d.get_string("symbol").unwrap(),
                    uqc: (
                        "uqc".to_string(),
                        d.get_string("uqc").unwrap().to_string().to_lowercase(),
                    )
                        .into(),
                    created: d.get_chrono_datetime("createdAt").unwrap_or_default(),
                    updated: d.get_chrono_datetime("updatedAt").unwrap_or_default(),
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
