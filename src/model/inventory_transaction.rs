use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct InventoryTransaction {
    pub date: String,
    pub act: bool,
    pub act_hide: bool,
    pub batch: Thing,
    pub branch: Thing,
    pub branch_name: String,
    pub inventory: Thing,
    pub inventory_name: String,
    pub inward: f64,
    pub outward: f64,
    pub unit_conv: f64,
    pub unit: Thing,
    pub unit_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type: Option<Thing>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cess_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opening: Option<bool>,
}

impl InventoryTransaction {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("inventory_transaction INDEX start");
        surrealdb
            .query("DEFINE INDEX inv ON TABLE inventory_transaction COLUMNS inventory")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX man ON TABLE inventory_transaction COLUMNS manufacturer")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX sec ON TABLE inventory_transaction COLUMNS section")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX date ON TABLE inventory_transaction COLUMNS date")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();

        surrealdb
            .query("DEFINE INDEX branch ON TABLE inventory_transaction COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX voucher_type_base ON TABLE inventory_transaction COLUMNS voucher_type_base")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("inventory_transaction INDEX end");
        println!("inventory_transaction download start");
        let mut cur = mongodb
            .collection::<Document>("inventory_transactions")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("inventory_transaction")
                .content(Self {
                    date: d.get_string("date").unwrap(),
                    inward: d._get_f64("inward").unwrap(),
                    outward: d._get_f64("outward").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    inventory_name: d.get_string("inventoryName").unwrap(),
                    manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
                    manufacturer_name: d.get_string("manufacturerName"),
                    act: d.get_bool("act").unwrap_or_default(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    batch: d.get_oid_to_thing("adjBatch", "batch").unwrap(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    branch_name: d.get_string("branchName").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap(),
                    unit: d.get_oid_to_thing("unitId", "unit").unwrap(),
                    unit_name: d.get_string("unitName").unwrap(),
                    ref_no: d.get_string("refNo"),
                    voucher_no: d.get_string("voucherNo"),
                    voucher_type_base: d.get_string("voucherType"),
                    voucher_type: d.get_oid_to_thing("voucherTypeId", "voucher_type"),
                    voucher: d.get_oid_to_thing("voucherId", "voucher"),
                    section: d.get_oid_to_thing("sectionId", "section"),
                    section_name: d.get_string("sectionName"),
                    contact: d
                        .get_oid_to_thing("customerId", "contact")
                        .or(d.get_oid_to_thing("vendorId", "contact")),
                    contact_name: d.get_string("customerName").or(d.get_string("vendorName")),
                    alt_account: d.get_oid_to_thing("altAccount", "account"),
                    alt_account_name: d.get_string("altAccountName"),
                    asset_amount: d._get_f64("assetAmount"),
                    taxable_amount: d._get_f64("taxableAmount"),
                    cgst_amount: d._get_f64("cgstAmount"),
                    cess_amount: d._get_f64("cessAmount"),
                    sgst_amount: d._get_f64("sgstAmount"),
                    igst_amount: d._get_f64("igstAmount"),
                    nlc: d._get_f64("nlc"),
                    is_opening: d.get_bool("isOpening").ok(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("inventory_transaction download end");
    }
}
