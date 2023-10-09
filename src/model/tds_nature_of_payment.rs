use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
#[derive(Debug, Serialize)]
pub struct TdsNatureOfPayment {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    pub ind_huf_rate: f64,
    pub ind_huf_rate_wo_pan: f64,
    pub other_deductee_rate: f64,
    pub other_deductee_rate_wo_pan: f64,
    pub threshold: f64,
}

impl TdsNatureOfPayment {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("tds_nature_of_payment INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE tds_nature_of_payment COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("tds_nature_of_payment INDEX end");
        println!("tds_nature_of_payment download start");
        let mut cur = mongodb
            .collection::<Document>("tds_nature_of_payments")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("tds_nature_of_payment")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "tds_nature_of_payment").unwrap(),
                    name: d.get_string("name").unwrap(),
                    section: d.get_string("section"),
                    display_name: d.get_string("displayName").unwrap(),
                    ind_huf_rate: d._get_f64("indHufRate").unwrap_or_default(),
                    ind_huf_rate_wo_pan: d._get_f64("indHufRateWoPan").unwrap_or_default(),
                    other_deductee_rate: d._get_f64("otherDeducteeRate").unwrap_or_default(),
                    other_deductee_rate_wo_pan: d
                        ._get_f64("otherDeducteeRateWoPan")
                        .unwrap_or_default(),
                    threshold: d._get_f64("threshold").unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("tds_nature_of_payment download end");
    }
}
