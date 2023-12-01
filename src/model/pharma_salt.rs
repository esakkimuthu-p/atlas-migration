use super::{doc, Database, Doc, Document, PostgresClient, StreamExt};

pub struct PharmaSalt;

impl PharmaSalt {
    pub async fn create(postgres: &PostgresClient, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("pharma_salts")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            postgres
                .execute(
                    "INSERT INTO pharma_salt (id,name,display_name) VALUES ($1, $2, $3)",
                    &[
                        &d.get_object_id("_id").unwrap().to_hex(),
                        &d.get_string("name").unwrap(),
                        &d.get_string("displayName").unwrap(),
                    ],
                )
                .await
                .unwrap();
        }
        println!("pharma_salt download end");
    }
}
