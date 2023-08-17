use super::{
    doc, Created, Database, DateTime, Deserialize, Doc, Document, HashSet, Serialize, StreamExt,
    Surreal, SurrealClient, Thing, Utc,
};
use mongodb::bson::from_document;

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
    pub val_name: String,
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

impl Inventory {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("inventory INDEX start");
        surrealdb
            .query("DEFINE INDEX validate_name ON TABLE inventory COLUMNS validate_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX barcodes ON TABLE inventory COLUMNS barcodes")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("inventory INDEX end");
        // inventories
        println!("inventory download start");
        let mut cur = mongodb
            .collection::<Document>("inventories")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let cess = d
                .get_document("cess")
                .ok()
                .and_then(|x| from_document::<InventoryCess>(x.clone()).ok());
            let mut units = Vec::new();
            for u_item in d.get_array_document("units").unwrap_or_default() {
                let unit = InventoryUnit {
                    unit: u_item.get_oid_to_thing("unitId", "unit").unwrap(),
                    unit_name: u_item.get_string("unitName").unwrap(),
                    conversion: u_item._get_f64("conversion").unwrap(),
                    preferred_for_purchase: u_item
                        .get_bool("preferredForPurchase")
                        .unwrap_or_default(),
                    preferred_for_sale: u_item.get_bool("preferredForPurchase").unwrap_or_default(),
                };
                units.push(unit);
            }

            let barcodes = d
                .get_array("barcodes")
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|x| {
                    x.as_str()
                        .is_some()
                        .then_some(x.as_str().unwrap_or_default().to_string())
                })
                .collect::<HashSet<String>>();
            let _created: Created = surrealdb
                .create("inventory")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "inventory").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    precision: d._get_f64("precision").unwrap() as u8,
                    head: d.get_oid_to_thing("head", "inventory_head").unwrap(),
                    allow_negative_stock: d.get_bool("allowNegativeStock").unwrap_or_default(),
                    tax: ("tax".to_string(), d.get_string("tax").unwrap()).into(), // gst0.1....
                    units,
                    cess,
                    barcodes: (!barcodes.is_empty()).then_some(barcodes),
                    hsn_code: d.get_string("hsnCode"),
                    description: d.get_string("description"),
                    section: d.get_oid_to_thing("sectionId", "section"),
                    section_name: d.get_string("sectionName"),
                    manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
                    manufacturer_name: d.get_string("manufacturerName"),
                    vendor: d.get_oid_to_thing("vendorId", "contact"),
                    vendor_name: d.get_string("vendorName"),
                    vendors: d.get_array_thing("vendors", "contact"),
                    salts: d.get_array_thing("salts", "pharma_salt"),
                    schedule_h: d.get_bool("scheduleH").ok(),
                    schedule_h1: d.get_bool("scheduleH1").ok(),
                    narcotics: d.get_bool("narcotics").ok(),
                    enable_expiry: d.get_bool("enableExpiry").unwrap_or_default(),
                    created: d.get_chrono_datetime("createdAt").unwrap_or_default(),
                    updated: d.get_chrono_datetime("updatedAt").unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("inventory download end");
    }
}