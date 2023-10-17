use super::{
    doc, BillAllocationApiInput, Database, Doc, Document, Serialize, StreamExt, Surreal,
    SurrealClient, Thing, TryStreamExt,
};

use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct AccountOpening {
    pub id: Thing,
    pub account: Thing,
    pub branch: Thing,
    pub credit: f64,
    pub debit: f64,
    pub bill_allocations: Option<Vec<BillAllocationApiInput>>,
}

impl AccountOpening {
    pub async fn set_account_opening(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let find_opts = FindOptions::builder()
            .projection(doc! {"defaultName": 1})
            .build();
        let accounts = mongodb
            .collection::<Document>("accounts")
            .find(doc! {"defaultName": {"$exists": true}}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let mut cur = mongodb
            .collection::<Document>("account_openings")
            .find(doc! {}, None)
            .await
            .unwrap();
        let find_opts = FindOptions::builder().projection(
            doc! {"txnId": 1, "amount": 1, "pending": {"$toString": "$pending"}, "refType": {"$toLower": "$refType"},"effDate": 1, "refNo": 1, "voucherNo": 1 }
        ).build();
        let bill_allocs = mongodb
            .collection::<Document>("bill_allocations")
            .find(
                doc! {"refType": "NEW", "voucherId": {"$exists": false}},
                find_opts,
            )
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let ba = bill_allocs
                .iter()
                .filter(|x| x.get_object_id("txnId").unwrap() == d.get_object_id("_id").unwrap());
            let mut bill_allocations = Vec::new();
            for (no, b) in ba.into_iter().enumerate() {
                bill_allocations.push(BillAllocationApiInput {
                    sno: no + 1,
                    pending: b.get_string("pending").unwrap(),
                    ref_type: b.get_string("refType").unwrap().to_lowercase(),
                    amount: b._get_f64("amount").unwrap_or_default(),
                    bill_date: b.get_string("effDate").unwrap(),
                    ref_no: b.get_string("refNo").unwrap_or_default(),
                })
            }
            let mut account = d.get_oid_to_thing("account", "account").unwrap();
            let account_doc = accounts
                .iter()
                .find(|x| x.get_object_id("_id").unwrap() == d.get_object_id("account").unwrap());
            if let Some(default_acc) = account_doc {
                account = (
                    "account".to_string(),
                    default_acc
                        .get_string("defaultName")
                        .unwrap()
                        .to_lowercase(),
                )
                    .into()
            }
            let input_data = AccountOpening {
                id: d.get_oid_to_thing("_id", "ac_txn").unwrap(),
                account,
                branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                credit: d._get_f64("credit").unwrap_or_default(),
                debit: d._get_f64("debit").unwrap_or_default(),
                bill_allocations: (!bill_allocations.is_empty()).then_some(bill_allocations),
            };
            let _created: String = surrealdb
                .query("fn::set_account_opening($data)")
                .bind(("data", input_data))
                .await
                .unwrap()
                .take::<Option<String>>(0)
                .unwrap()
                .unwrap();
        }
        println!("account_openings download end");
    }
}
