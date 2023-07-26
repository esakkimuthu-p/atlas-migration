use super::{Serialize, Thing};

#[derive(Debug, Serialize)]
pub struct AccountTransaction {
    pub date: String,
    pub account: Thing,
    pub account_name: String,
    pub account_type: String,
    pub act: bool,
    pub act_hide: bool,
    pub branch: Thing,
    pub branch_name: String,
    pub debit: f64,
    pub credit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eff_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opening: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_mode: Option<String>,
}
