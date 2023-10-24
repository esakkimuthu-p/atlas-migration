use super::{doc, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing};

#[derive(Debug, Serialize)]
pub struct BatchAllocationApiInput {
    sno: usize,
    batch: Thing,
    qty: f64,
    rate: f64,
    nlc: f64,
    unit_conv: f64,
    unit_precision: u8,
}

#[derive(Debug, Serialize)]
pub struct InventoryOpening {
    pub inventory: Thing,
    pub branch: Thing,
    pub batch_allocations: Vec<BatchAllocationApiInput>,
}

impl InventoryOpening {
    pub async fn set_inventory_opening(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("inventory_openings")
            .find(doc! {}, None)
            .await
            .unwrap();

        while let Some(Ok(d)) = cur.next().await {
            if let Some(inv_trns) = d.get_array_document("invTrns") {
                let mut batch_allocations = Vec::new();
                for (sno, inv_trn) in inv_trns.iter().enumerate() {
                    batch_allocations.push(BatchAllocationApiInput {
                        sno: sno + 1,
                        batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                        qty: inv_trn._get_f64("qty").unwrap_or_default(),
                        rate: inv_trn._get_f64("rate").unwrap_or_default(),
                        unit_conv: inv_trn._get_f64("unitConv").unwrap_or_default(),
                        unit_precision: inv_trn._get_f64("unitPrecision").unwrap_or_default() as u8,
                        nlc: inv_trn._get_f64("rate").unwrap_or_default()
                            / inv_trn._get_f64("unitConv").unwrap_or_default(),
                    });
                }
                if !batch_allocations.is_empty() {
                    let input_data = Self {
                        inventory: d.get_oid_to_thing("inventory", "inventory").unwrap(),
                        branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                        batch_allocations,
                    };
                    let _created: String = surrealdb
                        .query("fn::set_inventory_opening($data)")
                        .bind(("data", input_data))
                        .await
                        .unwrap()
                        .take::<Option<String>>(0)
                        .unwrap()
                        .unwrap();
                }
            }
        }
        println!("inventory_openings download end");
    }
}
