use mongodb::bson::from_document;

use super::{
    doc, AmountInfo, Created, Database, Doc, Document, Serialize, StreamExt, Surreal,
    SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct DiscountCode {
    pub id: Thing,
    pub code: String,
    pub discount_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    pub discount: AmountInfo,
}

impl DiscountCode {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("discount_code INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE discount_code COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("discount_code INDEX end");
        println!("discount_code download start");
        let mut cur = mongodb
            .collection::<Document>("discount_codes")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("discount_code")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "discount_code").unwrap(),
                    code: d.get_string("code").unwrap(),
                    discount_type: d.get_string("discountType").unwrap(),
                    bill_amount: d._get_f64("billAmount"),
                    expiry: d.get_string("expiry"),
                    discount: d
                        ._get_document("discount")
                        .and_then(|x| from_document::<AmountInfo>(x).ok())
                        .unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("discount_code download end");
    }
}
