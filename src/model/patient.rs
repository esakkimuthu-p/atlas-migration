use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};

#[derive(Debug, Serialize)]
pub struct Patient {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub val_name: String,
    pub customer: Thing,
    pub created: Datetime,
    pub updated: Datetime,
}

impl Patient {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("patient INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE patient COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX cust ON TABLE patient COLUMNS customer")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("patient INDEX end");
        println!("patient download start");
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
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    customer: d.get_oid_to_thing("customer", "contact").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
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
