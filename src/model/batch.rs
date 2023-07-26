use super::{DateTime, HashSet, Serialize, Thing, Utc};

#[derive(Debug, Serialize)]
pub struct Batch {
    pub id: Thing,
    pub inventory: Thing,
    pub inventory_name: String,
    pub branch: Thing,
    pub branch_name: String,
    pub barcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    pub last_inward: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_rate: Option<f64>,
    pub p_rate_tax_inc: bool,
    pub s_rate_tax_inc: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_nlc: Option<f64>,
    pub inward: f64,
    pub outward: f64,
    pub unit: Thing,
    pub unit_name: String,
    pub unit_conv: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<HashSet<Thing>>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
