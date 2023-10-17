use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct VendorItemMapping {
    pub vendor: Thing,
    pub inventory: Thing,
    pub v_inventory: String,
    pub v_unit: String,
    pub unit_conv: f64,
}

impl VendorItemMapping {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("vendor_item_mappings")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("vendor_item_mapping")
                .content(Self {
                    vendor: d.get_oid_to_thing("vendor", "contact").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    v_inventory: d.get_string("vInventory").unwrap(),
                    v_unit: d.get_string("vUnit").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap_or(1.0),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("vendor_item_mappings download end");
    }
}
