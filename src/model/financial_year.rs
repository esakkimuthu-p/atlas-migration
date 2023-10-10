use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct FinancialYear {
    pub id: Thing,
    pub fy_end: String,
    pub fy_start: String,
}

impl FinancialYear {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("financial_year download start");
        let mut cur = mongodb
            .collection::<Document>("financial_years")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("financial_year")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "financial_year").unwrap(),
                    fy_end: d.get_string("fEnd").unwrap(),
                    fy_start: d.get_string("fStart").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("financial_year download end");
    }
}
