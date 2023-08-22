use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct Doctor {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_no: Option<String>,
    pub created: Datetime,
    pub updated: Datetime,
}

impl Doctor {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("doctor INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE doctor COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("doctor INDEX end");
        println!("doctor download start");
        let mut cur = mongodb
            .collection::<Document>("doctors")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("doctor")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "doctor").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    license_no: d.get_string("licenseNo"),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("doctor download end");
    }
}
