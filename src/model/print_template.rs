use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct PrintTemplate {
    pub id: Thing,
    pub name: String,
    pub template: String,
    pub layout: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_mode: Option<String>,
    pub created: Datetime,
    pub updated: Datetime,
}

impl PrintTemplate {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("print_template INDEX start");
        surrealdb
            .query("DEFINE INDEX name ON TABLE print_template COLUMNS name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("print_template INDEX end");
        println!("print_template download start");
        let mut cur = mongodb
            .collection::<Document>("print_templates")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("print_template")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "print_template").unwrap(),
                    name: d.get_string("name").unwrap(),
                    voucher_mode: d.get_string("voucherMode"),
                    layout: d.get_string("layout").unwrap(),
                    template: d.get_string("template").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("print_template download end");
    }
}
