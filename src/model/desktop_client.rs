use super::{
    doc, Created, Database, Doc, Document, HashSet, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct DesktopClient {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub access: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<HashSet<Thing>>,
}

impl DesktopClient {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
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
                    display_name: d.get_string("displayName").unwrap(),
                    branches: d.get_array_thing("branches", "branch"),
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
