use super::{DateTime, Deserialize, HashSet, Serialize, Thing, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryUnit {
    pub unit: Thing,
    pub unit_name: String,
    pub conversion: f64,
    #[serde(default)]
    pub preferred_for_purchase: bool,
    #[serde(default)]
    pub preferred_for_sale: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryCess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_quantity: Option<f64>,
}
#[derive(Debug, Serialize)]
pub struct Inventory {
    pub id: Thing,
    pub name: String,
    pub validate_name: String,
    pub display_name: String,
    pub precision: u8,
    pub head: Thing,
    pub allow_negative_stock: bool,
    pub tax: Thing,
    pub units: Vec<InventoryUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cess: Option<InventoryCess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcodes: Option<HashSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<HashSet<Thing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salts: Option<HashSet<Thing>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub branch_details: Option<Vec<InventoryBranchDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_h: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_h1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narcotics: Option<bool>,
    #[serde(default)]
    pub enable_expiry: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
