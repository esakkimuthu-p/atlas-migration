use super::{
    doc, Created, Database, DateTime, Doc, Document, HashSet, Serialize, StreamExt, Surreal,
    SurrealClient, Thing, Utc,
};
#[derive(Debug, Serialize)]
pub struct DesktopClient {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    pub access: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<HashSet<Thing>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl DesktopClient {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("desktop_client INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE desktop_client COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("desktop_client INDEX end");
        println!("desktop_client download start");
        let mut cur = mongodb
            .collection::<Document>("desktop_clients")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("desktop_client")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "desktop_client").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    branches: d.get_array_thing("branches", "branch"),
                    created: d.get_chrono_datetime("createdAt").unwrap_or_default(),
                    updated: d.get_chrono_datetime("updatedAt").unwrap_or_default(),
                    access: d.get_bool("access").unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("desktop_client download end");
    }
}