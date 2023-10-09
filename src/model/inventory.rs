use super::{
    doc, serialize_tax_as_thing, AmountInfo, Created, Database, Datetime, Deserialize, Doc,
    Document, HashSet, Id, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::{bson::from_document, options::FindOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryUnit {
    pub unit: Thing,
    pub inventory: Thing,
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
pub struct CustomerDiscount {
    pub id: Thing,
    pub disc: AmountInfo,
}

#[derive(Debug, Serialize)]
pub struct PriceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_disc: Option<AmountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_margin: Option<AmountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrp_margin: Option<AmountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_profit_margin: Option<AmountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_customer_disc: Option<Vec<CustomerDiscount>>,
}

#[derive(Debug, Serialize)]
pub struct InventoryBranchDetail {
    pub branch: Thing,
    pub inventory: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_config: Option<PriceConfig>,
}

#[derive(Debug, Serialize)]
pub struct Inventory {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub precision: u8,
    pub branches: HashSet<Thing>,
    pub allow_negative_stock: bool,
    #[serde(serialize_with = "serialize_tax_as_thing")]
    pub gst_tax: String,
    pub unit: Thing,
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
    pub manufacturer: Option<Thing>,
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
        let find_opts = FindOptions::builder()
            .projection(doc! {"inventoryHead": 1})
            .build();
        let branches = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let cess = d
                ._get_document("cess")
                .and_then(|x| from_document::<InventoryCess>(x).ok());
            let mut units = Vec::new();
            let mut primary_unit = Thing {
                id: Id::rand(),
                tb: "unit".to_string(),
            };
            let id = d.get_oid_to_thing("_id", "inventory").unwrap();
            for u_item in d.get_array_document("units").unwrap_or_default() {
                let unit = InventoryUnit {
                    inventory: id.clone(),
                    unit: u_item.get_oid_to_thing("unitId", "unit").unwrap(),
                    conversion: u_item._get_f64("conversion").unwrap(),
                    preferred_for_purchase: u_item
                        .get_bool("preferredForPurchase")
                        .unwrap_or_default(),
                    preferred_for_sale: u_item.get_bool("preferredForPurchase").unwrap_or_default(),
                };
                if unit.conversion == 1.0 {
                    primary_unit = unit.unit;
                } else {
                    units.push(unit);
                }
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
            let inv_head = d.get_object_id("head").unwrap();
            let inv_branches = branches
                .iter()
                .filter_map(|x| {
                    (x.get_object_id("inventoryHead").unwrap() == inv_head)
                        .then_some(x.get_oid_to_thing("_id", "branch").unwrap())
                })
                .collect::<HashSet<Thing>>();

            let _created: Created = surrealdb
                .create("inventory")
                .content(Self {
                    id: id.clone(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    precision: d._get_f64("precision").unwrap() as u8,
                    branches: inv_branches,
                    allow_negative_stock: d.get_bool("allowNegativeStock").unwrap_or_default(),
                    gst_tax: d.get_string("tax").unwrap(),
                    unit: primary_unit,
                    cess,
                    barcodes: (!barcodes.is_empty()).then_some(barcodes),
                    hsn_code: d.get_string("hsnCode"),
                    description: d.get_string("description"),
                    section: d.get_oid_to_thing("sectionId", "section"),
                    manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
                    vendors: d.get_array_thing("vendors", "contact"),
                    salts: d.get_array_thing("salts", "pharma_salt"),
                    schedule_h: d.get_bool("scheduleH").ok(),
                    schedule_h1: d.get_bool("scheduleH1").ok(),
                    narcotics: d.get_bool("narcotics").ok(),
                    enable_expiry: d.get_bool("enableExpiry").unwrap_or_default(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
            if let Some(branch_details) = d.get_array_document("branchDetails") {
                for br_detail in branch_details {
                    let s_disc = br_detail
                        ._get_document("sDisc")
                        .and_then(|x| from_document::<AmountInfo>(x).ok());
                    let cost_margin = br_detail
                        ._get_document("costMargin")
                        .and_then(|x| from_document::<AmountInfo>(x).ok());
                    let mrp_margin = br_detail
                        ._get_document("mrpMargin")
                        .and_then(|x| from_document::<AmountInfo>(x).ok());
                    let min_profit_margin = br_detail
                        ._get_document("minProfitMargin")
                        .and_then(|x| from_document::<AmountInfo>(x).ok());
                    let rack = br_detail
                        ._get_document("rack")
                        .and_then(|x| x.get_oid_to_thing("id", "rack"));
                    let mut s_customer_disc: Vec<CustomerDiscount> = Vec::new();
                    if let Some(c_s_disc) = br_detail.get_array_document("sCustomerDisc") {
                        for c_disc in c_s_disc {
                            let disc = CustomerDiscount {
                                id: c_disc.get_oid_to_thing("id", "customer_group").unwrap(),
                                disc: from_document::<AmountInfo>(
                                    c_disc._get_document("disc").unwrap(),
                                )
                                .unwrap(),
                            };
                            s_customer_disc.push(disc);
                        }
                    }
                    let mut price_config = None;
                    if !s_customer_disc.is_empty()
                        || s_disc.is_some()
                        || cost_margin.is_some()
                        || min_profit_margin.is_some()
                    {
                        price_config = Some(PriceConfig {
                            s_disc,
                            cost_margin,
                            mrp_margin,
                            min_profit_margin,
                            s_customer_disc: (!s_customer_disc.is_empty())
                                .then_some(s_customer_disc),
                        });
                    }
                    let _created: Created = surrealdb
                        .create("inv_branch_detail")
                        .content(InventoryBranchDetail {
                            branch: br_detail.get_oid_to_thing("branch", "branch").unwrap(),
                            inventory: id.clone(),
                            rack,
                            price_config,
                            vendor: d.get_oid_to_thing("vendorId", "contact"),
                        })
                        .await
                        .unwrap()
                        .first()
                        .cloned()
                        .unwrap();
                }
            }
        }
        println!("inventory download end");
    }
}
