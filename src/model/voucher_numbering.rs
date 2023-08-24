use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct VoucherNumbering {
    pub branch: Thing,
    pub f_year: Thing,
    pub sequence: u32,
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
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("voucher_numbering")
                .content(Self {
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    f_year: d.get_oid_to_thing("fYear", "financial_year").unwrap(),
                    sequence: d._get_f64("sequence").unwrap() as u32,
                    voucher_type: d.get_oid_to_thing("voucherTypeId", "voucher_type").unwrap(),
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
