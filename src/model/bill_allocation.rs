use super::{
    doc, serialize_round_2, Created, Database, Datetime, Doc, Document, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct BillAllocation {
    pub account: Thing,
    pub account_name: String,
    pub account_type: Thing,
    pub branch: Thing,
    pub branch_name: String,
    pub date: Datetime,
    pub eff_date: Datetime,
    #[serde(serialize_with = "serialize_round_2")]
    pub amount: f64,
    pub txn_id: Thing,
    pub pending: Thing,
    pub ref_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher: Option<Thing>,
    pub updated: Datetime,
}
impl BillAllocation {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("bill_allocation INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE bill_allocation COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("bill_allocation INDEX end");
        println!("bill_allocation download start");
        let mut cur = mongodb
            .collection::<Document>("bill_allocations")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("bill_allocation")
                .content(Self {
                    account: d.get_oid_to_thing("account", "account").unwrap(),
                    account_name: d.get_string("accountName").unwrap(),
                    account_type: (
                        "account_type".to_string(),
                        d.get_string("accountType").unwrap().to_lowercase(),
                    )
                        .into(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    branch_name: d.get_string("branchName").unwrap(),
                    date: d.get_surreal_datetime_from_str("date").unwrap(),
                    eff_date: d
                        .get_surreal_datetime_from_str("effDate")
                        .unwrap_or(d.get_surreal_datetime_from_str("date").unwrap()),
                    amount: d._get_f64("amount").unwrap_or_default(),
                    txn_id: d.get_oid_to_thing("txnId", "txn_id").unwrap(),
                    pending: d.get_oid_to_thing("pending", "pending").unwrap(),
                    ref_type: d.get_string("refType").unwrap(),
                    voucher: d.get_oid_to_thing("voucherId", "voucher"),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("bill_allocation download end");
    }
}
