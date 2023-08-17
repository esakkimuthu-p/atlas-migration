use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct AccountTransaction {
    pub date: String,
    pub account: Thing,
    pub account_name: String,
    pub account_type: String,
    pub act: bool,
    pub act_hide: bool,
    pub branch: Thing,
    pub branch_name: String,
    pub debit: f64,
    pub credit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eff_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opening: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_type: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_mode: Option<String>,
}

impl AccountTransaction {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("account_transaction INDEX start");
        surrealdb
            .query("DEFINE INDEX acc ON TABLE account_transaction COLUMNS account")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX acc_type ON TABLE account_transaction COLUMNS account_type")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX voucher_id ON TABLE account_transaction COLUMNS voucher")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX date ON TABLE account_transaction COLUMNS date")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();

        surrealdb
            .query("DEFINE INDEX branch ON TABLE account_transaction COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb.query(
            "DEFINE INDEX voucher_type_base ON TABLE account_transaction COLUMNS voucher_type_base",
        )
        .await
        .unwrap()
        .take::<Option<()>>(0)
        .unwrap();
        println!("account_transaction INDEX end");
        println!("account_transaction download start");
        let mut cur = mongodb
            .collection::<Document>("account_transactions")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("account_transaction")
                .content(Self {
                    date: d.get_string("date").unwrap(),
                    debit: d._get_f64("debit").unwrap(),
                    credit: d._get_f64("credit").unwrap(),
                    account: d.get_oid_to_thing("account", "account").unwrap(),
                    account_name: d.get_string("accountName").unwrap(),
                    account_type: d.get_string("accountType").unwrap(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    branch_name: d.get_string("branchName").unwrap(),
                    act: d.get_bool("act").unwrap_or_default(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    alt_account: d.get_oid_to_thing("altAccount", "account"),
                    alt_account_name: d.get_string("altAccountName"),
                    ref_no: d.get_string("refNo"),
                    voucher_no: d.get_string("voucherNo"),
                    voucher_type_base: d.get_string("voucherType"),
                    voucher_type: d.get_oid_to_thing("voucherTypeId", "voucher_type"),
                    voucher: d.get_oid_to_thing("voucherId", "voucher"),
                    is_opening: d.get_bool("isOpening").ok(),
                    eff_date: d.get_string("effDate"),
                    voucher_mode: d.get_string("voucherMode"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("account_transaction download end");
    }
}
