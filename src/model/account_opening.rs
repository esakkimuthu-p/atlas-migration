use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

use super::{
    doc, serialize_round_2, AccountTransaction, BankTransaction, Created, Database, Datetime, Doc,
    Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct AccountOpening {
    pub id: Thing,
    pub account: Thing,
    pub branch: Thing,
    #[serde(serialize_with = "serialize_round_2")]
    pub credit: f64,
    #[serde(serialize_with = "serialize_round_2")]
    pub debit: f64,
    pub act_hide: bool,
    pub act: bool,
    pub updated_at: Datetime,
}

impl AccountOpening {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("account_opening INDEX start");
        surrealdb
            .query("DEFINE INDEX br ON TABLE account_opening COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX acc ON TABLE account_opening COLUMNS account")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("account_opening INDEX end");
        println!("account_opening download start");
        let acc_find_opts = FindOptions::builder()
            .projection(doc! {"displayName": 1, "defaultName": 1, "accountType": 1})
            .build();
        let accounts = mongodb
            .collection::<Document>("accounts")
            .find(doc! {}, acc_find_opts.clone())
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let branches = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, acc_find_opts)
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
        while let Some(Ok(d)) = cur.next().await {
            let account_doc = accounts
                .iter()
                .find(|x| x.get_object_id("_id").unwrap() == d.get_object_id("account").unwrap())
                .unwrap();
            let branch_name = branches
                .iter()
                .find_map(|x| {
                    (x.get_object_id("_id").unwrap() == d.get_object_id("branch").unwrap())
                        .then_some(x.get_string("displayName").unwrap())
                })
                .unwrap();
            let (account, account_name, account_type): (Thing, String, String) =
                if let Some(default_name) = account_doc.get_string("defaultName") {
                    (
                        ("account".to_string(), default_name.to_lowercase()).into(),
                        account_doc.get_string("displayName").unwrap(),
                        account_doc
                            .get_string("accountType")
                            .unwrap()
                            .to_lowercase(),
                    )
                } else {
                    (
                        d.get_oid_to_thing("account", "account").unwrap(),
                        account_doc.get_string("displayName").unwrap(),
                        account_doc
                            .get_string("accountType")
                            .unwrap()
                            .to_lowercase(),
                    )
                };
            let _created: Created = surrealdb
                .create("account_opening")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "account_opening").unwrap(),
                    account: account.clone(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    credit: d._get_f64("credit").unwrap_or_default(),
                    debit: d._get_f64("debit").unwrap_or_default(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    act: d.get_bool("act").unwrap_or_default(),
                    updated_at: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();

            if ["bank_account".to_string(), "bank_od_account".to_string()].contains(&account_type) {
                let _created: Created = surrealdb
                    .create("bank_txn")
                    .content(BankTransaction {
                        date: Datetime::default(), // org book begin date
                        txn: 
                        debit: d._get_f64("debit").unwrap_or_default(),
                        credit: d._get_f64("credit").unwrap_or_default(),
                        account: account.clone(),
                        account_name: account_name.clone(),
                        account_type: ("account_type".to_string(), account_type.clone()).into(),
                        branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                        branch_name: branch_name.clone(),
                        alt_account: None,
                        alt_account_name: None,
                        inst_no: None,
                        in_favour_of: None,
                        voucher: None,
                        bank_date: None,
                        inst_date: None,
                    })
                    .await
                    .unwrap()
                    .first()
                    .cloned()
                    .unwrap();
            }
            let _created: Created = surrealdb
                .create("account_transaction")
                .content(AccountTransaction {
                    date: Datetime::default(), // org book begin date
                    debit: d._get_f64("debit").unwrap_or_default(),
                    credit: d._get_f64("credit").unwrap_or_default(),
                    account,
                    account_name,
                    account_type: ("account_type".to_string(), account_type).into(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    branch_name: branch_name.clone(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    act: d.get_bool("act").unwrap_or_default(),
                    alt_account: None,
                    alt_account_name: None,
                    ref_no: None,
                    base_voucher_type: None,
                    voucher: None,
                    is_opening: Some(true),
                    is_default: None,
                    gst_tax: None,
                    voucher_mode: None,
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("account_opening download end");
    }
}
