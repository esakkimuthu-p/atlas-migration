use super::{
    doc, serialize_opt_round_2, serialize_opt_round_4, serialize_round_2, serialize_round_4,
    Datetime, Document, Serialize, Thing,
};

#[derive(Debug, Serialize)]
pub struct InventoryTransaction {
    pub date: Datetime,
    pub act: bool,
    pub act_hide: bool,
    pub batch: Thing,
    pub branch: Thing,
    pub branch_name: String,
    pub inventory: Thing,
    pub gst_tax: Option<Thing>,
    pub inventory_name: String,
    #[serde(serialize_with = "serialize_round_4")]
    pub inward: f64,
    #[serde(serialize_with = "serialize_round_4")]
    pub outward: f64,
    #[serde(serialize_with = "serialize_round_2")]
    pub rate: f64,
    #[serde(serialize_with = "serialize_round_4")]
    pub qty: f64,
    #[serde(serialize_with = "serialize_round_4")]
    pub unit_conv: f64,
    pub unit_precision: u8,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_4"
    )]
    pub free_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_voucher_type: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub asset_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub taxable_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub cgst_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub cess_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub sgst_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub igst_amount: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opening: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_inc: Option<Thing>,
}

// impl InventoryTransaction {
//     pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
//         println!("inventory_transaction INDEX start");
//         surrealdb
//             .query("DEFINE INDEX inv ON TABLE inventory_transaction COLUMNS inventory")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX man ON TABLE inventory_transaction COLUMNS manufacturer")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX sec ON TABLE inventory_transaction COLUMNS section")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX date ON TABLE inventory_transaction COLUMNS date")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();

//         surrealdb
//             .query("DEFINE INDEX branch ON TABLE inventory_transaction COLUMNS branch")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX voucher_type_base ON TABLE inventory_transaction COLUMNS voucher_type_base")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         println!("inventory_transaction INDEX end");
//         println!("inventory_transaction download start");
//         let mut cur = mongodb
//             .collection::<Document>("inventory_transactions")
//             .find(doc! {}, None)
//             .await
//             .unwrap();
//         while let Some(Ok(d)) = cur.next().await {
//             let _created: Created = surrealdb
//                 .create("inventory_transaction")
//                 .content(Self {
//                     date: d.get_surreal_datetime_from_str("date").unwrap(),
//                     inward: d._get_f64("inward").unwrap(),
//                     outward: d._get_f64("outward").unwrap(),
//                     inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
//                     inventory_name: d.get_string("inventoryName").unwrap(),
//                     manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
//                     manufacturer_name: d.get_string("manufacturerName"),
//                     act: d.get_bool("act").unwrap_or_default(),
//                     act_hide: d.get_bool("actHide").unwrap_or_default(),
//                     batch: d.get_oid_to_thing("adjBatch", "batch").unwrap(),
//                     branch: d.get_oid_to_thing("branch", "branch").unwrap(),
//                     branch_name: d.get_string("branchName").unwrap(),
//                     unit_conv: d._get_f64("unitConv").unwrap(),
//                     ref_no: d.get_string("refNo"),
//                     voucher: d.get_oid_to_thing("voucherId", "voucher"),
//                     section: d.get_oid_to_thing("sectionId", "section"),
//                     section_name: d.get_string("sectionName"),
//                     contact: d
//                         .get_oid_to_thing("customerId", "contact")
//                         .or(d.get_oid_to_thing("vendorId", "contact")),
//                     contact_name: d.get_string("customerName").or(d.get_string("vendorName")),
//                     // alt_account: d.get_oid_to_thing("altAccount", "account"),
//                     // alt_account_name: d.get_string("altAccountName"),
//                     asset_amount: d._get_f64("assetAmount"),
//                     taxable_amount: d._get_f64("taxableAmount"),
//                     cgst_amount: d._get_f64("cgstAmount"),
//                     cess_amount: d._get_f64("cessAmount"),
//                     sgst_amount: d._get_f64("sgstAmount"),
//                     igst_amount: d._get_f64("igstAmount"),
//                     nlc: d._get_f64("nlc"),
//                     is_opening: d.get_bool("isOpening").ok(),
//                 })
//                 .await
//                 .unwrap()
//                 .first()
//                 .cloned()
//                 .unwrap();
//         }
//         println!("inventory_transaction download end");
//     }
// }
