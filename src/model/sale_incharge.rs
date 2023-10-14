use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct SaleIncharge {
    pub id: Thing,
    pub name: String,
    pub code: String,
}

impl SaleIncharge {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("sale_incharge download start");
        let mut cur = mongodb
            .collection::<Document>("sale_incharges")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("sale_incharge")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "sale_incharge").unwrap(),
                    name: d.get_string("name").unwrap(),
                    code: d.get_string("code").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("sale_incharge download end");
    }
}
