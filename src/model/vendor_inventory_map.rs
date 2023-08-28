use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
#[derive(Debug, Serialize)]
pub struct VendorItemMap {
    pub vendor: Thing,
    pub v_inventory: String,
    pub v_unit: String,
    pub inventory: Thing,
    pub unit_conv: f64,
}

#[derive(Debug, Serialize)]
pub struct VendorBillMap {
    pub vendor: Thing,
    pub mapping: String,
}

impl VendorItemMap {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("vendor_item_mapping INDEX start");
        surrealdb
            .query("DEFINE INDEX vendor ON TABLE vendor_item_mapping COLUMNS vendor")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("vendor_item_mapping INDEX end");
        println!("vendor_item_mapping download start");
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
                    v_inventory: d.get_string("vInventory").unwrap(),
                    v_unit: d.get_string("vUnit").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap_or(1.0),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("vendor_item_mapping download end");
    }
}

impl VendorBillMap {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("vendor_bill_mapping INDEX start");
        surrealdb
            .query("DEFINE INDEX vendor ON TABLE vendor_bill_mapping COLUMNS vendor")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("vendor_bill_mapping INDEX end");
        println!("vendor_bill_mapping download start");
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
        println!("vendor_bill_mapping download end");
    }
}
