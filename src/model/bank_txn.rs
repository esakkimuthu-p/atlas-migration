use super::{doc, serialize_round_2, Datetime, Serialize, Thing};

#[derive(Debug, Serialize)]
pub struct BankTransaction {
    pub date: Datetime,
    pub branch: Thing,
    pub branch_name: String,
    pub account: Thing,
    pub account_name: String,
    pub account_type: Thing,
    pub txn: Thing,
    #[serde(serialize_with = "serialize_round_2")]
    pub debit: f64,
    #[serde(serialize_with = "serialize_round_2")]
    pub credit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_date: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_favour_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_date: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher: Option<Thing>,
}

// impl BankTransaction {
//     pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
//         println!("bank_txn INDEX start");
//         surrealdb
//             .query("DEFINE INDEX acc ON TABLE bank_txn COLUMNS account")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX acc_type ON TABLE bank_txn COLUMNS account_type")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX voucher_id ON TABLE bank_txn COLUMNS voucher")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         surrealdb
//             .query("DEFINE INDEX date ON TABLE bank_txn COLUMNS date")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();

//         surrealdb
//             .query("DEFINE INDEX branch ON TABLE bank_txn COLUMNS branch")
//             .await
//             .unwrap()
//             .take::<Option<()>>(0)
//             .unwrap();
//         println!("bank_txn INDEX end");
//         println!("bank_txn download start");
//         let mut cur = mongodb
//             .collection::<Document>("bank_transactions")
//             .find(doc! {}, None)
//             .await
//             .unwrap();
//         while let Some(Ok(d)) = cur.next().await {
//             let _created: Created = surrealdb
//                 .create("bank_txn")
//                 .content(Self {
//                     date: d.get_surreal_datetime_from_str("date").unwrap(),
//                     debit: d._get_f64("debit").unwrap_or_default(),
//                     credit: d._get_f64("credit").unwrap_or_default(),
//                     account: d.get_oid_to_thing("account", "account").unwrap(),
//                     account_name: d.get_string("accountName").unwrap(),
//                     account_type: (
//                         "account_type".to_string(),
//                         d.get_string("accountType").unwrap().to_lowercase(),
//                     )
//                         .into(),
//                     branch: d.get_oid_to_thing("branch", "branch").unwrap(),
//                     branch_name: d.get_string("branchName").unwrap(),
//                     alt_account: d.get_oid_to_thing("altAccount", "account"),
//                     alt_account_name: d.get_string("altAccountName"),
//                     inst_no: d.get_string("instNo"),
//                     in_favour_of: d.get_string("inFavourOf"),
//                     voucher: d.get_oid_to_thing("voucherId", "voucher"),
//                     bank_date: d.get_surreal_datetime_from_str("bankDate"),
//                     inst_date: d.get_surreal_datetime_from_str("instDate"),
//                 })
//                 .await
//                 .unwrap()
//                 .first()
//                 .cloned()
//                 .unwrap();
//         }
//         println!("bank_txn download end");
//     }
// }
