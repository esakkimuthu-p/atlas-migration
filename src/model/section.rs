use super::{
    doc, Created, Database, DateTime, Doc, Document, HashSet, Serialize, StreamExt, Surreal,
    SurrealClient, Thing, Utc,
};

#[derive(Debug, Serialize)]
pub struct Section {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<HashSet<Thing>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Section {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("section INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE section COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("section INDEX end");
        println!("section download start");
        let mut cur = mongodb
            .collection::<Document>("sections")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("section")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "section").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    parent: d.get_oid_to_thing("parentSection", "section"),
                    parents: d.get_array_thing("parents", "section"),
                    created: d.get_chrono_datetime("createdAt").unwrap_or_default(),
                    updated: d.get_chrono_datetime("updatedAt").unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("section download end");
    }
}
