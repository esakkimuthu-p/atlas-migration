use super::{
    doc, Created, Database, Datetime, Doc, Document, InventoryTransaction, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct InventoryOpening {
    pub inventory: Thing,
    pub branch: Thing,
    pub act_hide: bool,
    pub act: bool,
    pub updated_at: Datetime,
}

impl InventoryOpening {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("inventory_opening INDEX start");
        surrealdb
            .query("DEFINE INDEX br ON TABLE inventory_opening COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX inv ON TABLE inventory_opening COLUMNS inventory")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("inventory_opening INDEX end");
        println!("inventory_opening download start");
        let mut cur = mongodb
            .collection::<Document>("inventory_openings")
            .find(doc! {}, None)
            .await
            .unwrap();
        let inv_find_opts = FindOptions::builder()
        .projection(doc! { "displayName": 1, "sectionId": 1, "sectionName": 1, "manufacturerId":1, "manufacturerName": 1, "tax": 1 })
        .build();
        let inventories = mongodb
            .collection::<Document>("inventories")
            .find(doc! {}, inv_find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let branches = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, None)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let branch = d.get_oid_to_thing("branch", "branch").unwrap();
            let inventory = d.get_oid_to_thing("inventory", "inventory").unwrap();
            let act_hide = d.get_bool("actHide").unwrap_or_default();
            let act = d.get_bool("act").unwrap_or_default();
            let _created: Created = surrealdb
                .create("inventory_opening")
                .content(Self {
                    inventory: inventory.clone(),
                    branch: branch.clone(),
                    act_hide,
                    act,
                    updated_at: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
            let inventory_doc = inventories
                .iter()
                .find(|x| x.get_object_id("_id").unwrap() == d.get_object_id("inventory").unwrap())
                .unwrap();
            let branch_doc = branches
                .iter()
                .find(|x| x.get_object_id("_id").unwrap() == d.get_object_id("branch").unwrap())
                .unwrap();
            if let Some(inv_trns) = d.get_array_document("invTrns") {
                for inv_trn in inv_trns {
                    let qty = inv_trn._get_f64("qty").unwrap_or_default();
                    let unit_conv = inv_trn._get_f64("unitConv").unwrap();
                    let rate = inv_trn._get_f64("rate").unwrap();
                    let _created: Created = surrealdb
                        .create("inventory_transaction")
                        .content(InventoryTransaction {
                            date: Datetime::default(),
                            inward: qty * unit_conv,
                            outward: 0.0,
                            free_qty: None,
                            qty,
                            rate,
                            unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                            branch: branch.clone(),
                            branch_name: branch_doc.get_string("displayName").unwrap().clone(),
                            gst_tax: (
                                "gst_tax".to_string(),
                                inventory_doc.get_string("tax").unwrap(),
                            )
                                .into(),
                            disc: None,
                            act,
                            act_hide,
                            ref_no: None,
                            base_voucher_type: None,
                            voucher: None,
                            is_opening: Some(true),
                            batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                            inventory: inventory.clone(),
                            inventory_name: inventory_doc.get_string("displayName").unwrap(),
                            unit_conv,
                            section: inventory_doc.get_oid_to_thing("sectionId", "section"),
                            section_name: inventory_doc.get_string("sectionName"),
                            manufacturer: inventory_doc
                                .get_oid_to_thing("manufacturerId", "manufacturer"),
                            manufacturer_name: inventory_doc.get_string("manufacturerName"),
                            contact: None,
                            contact_name: None,
                            asset_amount: Some(rate * qty),
                            taxable_amount: None,
                            cgst_amount: None,
                            cess_amount: None,
                            sgst_amount: None,
                            igst_amount: None,
                            nlc: Some(rate / unit_conv),
                        })
                        .await
                        .unwrap()
                        .first()
                        .cloned()
                        .unwrap();
                }
            }
        }
        println!("inventory_opening download end");
    }
}
