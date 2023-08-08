use super::{DateTime, HashSet, Serialize, Thing, Utc};

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: Thing,
    pub name: String,
    pub val_name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_favour_of: Option<String>,
    pub account_type: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sac_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<HashSet<Thing>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tds_nature_of_payment: Option<Thing>,
    pub hide: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
