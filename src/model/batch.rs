use super::{
    doc, serialize_opt_round_2, serialize_round_4, Created, Database, Datetime, Doc, Document,
    HashSet, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

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
    pub expiry: Option<Datetime>,
    pub last_inward: Datetime,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub mrp: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub s_rate: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub p_rate: Option<f64>,
    pub p_rate_tax_inc: bool,
    pub s_rate_tax_inc: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub avg_nlc: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub min_nlc: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub max_nlc: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub first_nlc: Option<f64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_round_2"
    )]
    pub last_nlc: Option<f64>,
    #[serde(serialize_with = "serialize_round_4")]
    pub inward: f64,
    #[serde(serialize_with = "serialize_round_4")]
    pub outward: f64,
    pub unit: Thing,
    pub unit_name: String,
    #[serde(serialize_with = "serialize_round_4")]
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
    pub created: Datetime,
    pub updated: Datetime,
}

impl Batch {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("batch INDEX start");
        surrealdb
            .query("DEFINE INDEX branch ON TABLE batch COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX barcode ON TABLE batch COLUMNS barcode")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX inv ON TABLE batch COLUMNS inventory")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX sec ON TABLE batch COLUMNS section")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX man ON TABLE batch COLUMNS manufacturer")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("batch INDEX end");
        println!("batch download start");
        let mut cur = mongodb
            .collection::<Document>("batches")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("batch")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "batch").unwrap(),
                    barcode: d.get_object_id("barcode").unwrap().to_hex(),
                    batch_no: d.get_string("batchNo"),
                    last_inward: d.get_surreal_datetime_from_str("lastInward").unwrap(),
                    inward: d._get_f64("inward").unwrap(),
                    outward: d._get_f64("outward").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    inventory_name: d.get_string("inventoryName").unwrap(),
                    expiry: d.get_surreal_datetime_from_str("expiry"),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    branch_name: d.get_string("branchName").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap(),
                    unit: d.get_oid_to_thing("unitId", "unit").unwrap(),
                    unit_name: d.get_string("unitName").unwrap(),
                    section: d.get_oid_to_thing("sectionId", "section"),
                    section_name: d.get_string("sectionName"),
                    manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
                    manufacturer_name: d.get_string("manufacturerName"),
                    mrp: d._get_f64("mrp"),
                    s_rate: d._get_f64("sRate"),
                    p_rate: d._get_f64("pRate"),
                    avg_nlc: d._get_f64("avgNlc"),
                    first_nlc: d._get_f64("firstNlc"),
                    last_nlc: d._get_f64("lastNlc"),
                    max_nlc: d._get_f64("maxNlc"),
                    min_nlc: d._get_f64("minNlc"),
                    p_rate_tax_inc: d.get_bool("pRateTaxInc").unwrap_or_default(),
                    s_rate_tax_inc: d.get_bool("sRateTaxInc").unwrap_or(true),
                    vendors: d.get_array_thing("vendors", "contact"),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("batch download end");
    }
}
