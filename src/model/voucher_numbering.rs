use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct VoucherNumbering {
    pub branch: Thing,
    pub f_year: Thing,
    pub seq: u32,
    pub voucher_type: Thing,
}

impl VoucherNumbering {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("voucher_numbering download start");
        let mut cur = mongodb
            .collection::<Document>("voucher_numberings")
            .find(doc! {}, None)
            .await
            .unwrap();
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1})
            .build();
        let voucher_types = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let voucher_type = voucher_types
                .iter()
                .find_map(|x| {
                    (x.get_bool("default").unwrap_or_default()
                        && x.get_object_id("_id").unwrap()
                            == d.get_object_id("voucherTypeId").unwrap())
                    .then_some(
                        (
                            "voucher_type".to_string(),
                            x.get_string("voucherType").unwrap().to_lowercase(),
                        )
                            .into(),
                    )
                })
                .unwrap_or(d.get_oid_to_thing("voucherTypeId", "voucher_type").unwrap());
            let _created: Created = surrealdb
                .create("voucher_numbering")
                .content(Self {
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    f_year: d.get_oid_to_thing("fYear", "financial_year").unwrap(),
                    seq: d._get_f64("sequence").unwrap() as u32,
                    voucher_type,
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("voucher_numbering download end");
    }
}
