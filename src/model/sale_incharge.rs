use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct SaleIncharge {
    pub id: Thing,
    pub name: String,
    pub code: String,
}

impl SaleIncharge {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("sale_incharge INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE sale_incharge COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX code ON TABLE sale_incharge COLUMNS code")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("sale_incharge INDEX end");
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
