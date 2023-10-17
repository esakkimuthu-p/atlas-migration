use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct VendorBillMapping {
    pub vendor: Thing,
    pub mapping: String,
}

impl VendorBillMapping {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("vendor_bill_mappings")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("vendor_bill_mapping")
                .content(Self {
                    vendor: d.get_oid_to_thing("vendor", "contact").unwrap(),
                    mapping: d.get_string("mapping").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("vendor_bill_mappings download end");
    }
}
