use super::{Database, Doc, Document, PostgresClient, StreamExt};
pub struct Rack;

impl Rack {
    pub async fn create(postgres: &PostgresClient, mongodb: &Database, filter: Option<Document>) {
        let mut cur = mongodb
            .collection::<Document>("racks")
            .find(filter.unwrap_or_default(), None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            postgres
                .execute(
                    "INSERT INTO rack (id,name,display_name) VALUES ($1, $2, $3)",
                    &[
                        &d.get_object_id("_id").unwrap().to_hex(),
                        &d.get_string("name").unwrap(),
                        &d.get_string("displayName").unwrap(),
                    ],
                )
                .await
                .unwrap();
        }
        println!("rack download end");
    }
}
