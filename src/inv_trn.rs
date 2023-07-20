use serde::Serialize;
use surrealdb::sql::Thing;

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
