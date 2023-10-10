use super::{
    doc, Created, Database, Doc, Document, HashSet, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};

#[derive(Debug, Serialize)]
pub struct Batch {
    pub id: Thing,
    pub inventory: Thing,
    pub branch: Thing,
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
    pub manufacturer: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<HashSet<Thing>>,
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
                    last_inward: d.get_string("lastInward").unwrap(),
                    inward: d._get_f64("inward").unwrap(),
                    outward: d._get_f64("outward").unwrap(),
                    inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                    expiry: d.get_string("expiry"),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    unit_conv: d._get_f64("unitConv").unwrap(),
                    unit: d.get_oid_to_thing("unitId", "unit").unwrap(),
                    unit_name: d.get_string("unitName").unwrap(),
                    section: d.get_oid_to_thing("sectionId", "section"),
                    manufacturer: d.get_oid_to_thing("manufacturerId", "manufacturer"),
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
