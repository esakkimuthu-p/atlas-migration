use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct Manufacturer {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    pub created: Datetime,
    pub updated: Datetime,
}

impl Manufacturer {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("manufacturer INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE manufacturer COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("manufacturer INDEX end");
        println!("manufacturer download start");
        let mut cur = mongodb
            .collection::<Document>("manufacturers")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("manufacturer")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "manufacturer").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                    email: d.get_string("email"),
                    mobile: d.get_string("mobile"),
                    telephone: d.get_string("telephone"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("manufacturer download end");
    }
}
