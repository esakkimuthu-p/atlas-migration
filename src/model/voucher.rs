use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
use crate::model::AccountTransaction;
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct Voucher {
    pub id: Thing,
    pub branch: Thing,
    pub voucher_type: Thing,
    pub base_voucher_type: Thing,
    pub act: bool,
    pub act_hide: bool,
    pub date: Datetime,
    pub eff_date: Datetime,
    pub voucher_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: f64,
    pub created: Datetime,
    pub updated: Datetime,
}

fn get_alt_accounts(
    ac_trns: &[Document],
    accounts: &[Document],
) -> (Option<(Thing, String)>, Option<(Thing, String)>) {
    let mut alt_trns = ac_trns
        .iter()
        .cloned()
        .filter(|x| x.get_string("accountType").unwrap_or_default() != "STOCK")
        .collect::<Vec<Document>>();
    alt_trns.sort_by(|a, b| {
        b._get_f64("debit")
            .unwrap_or_default()
            .total_cmp(&a._get_f64("debit").unwrap_or_default())
    });
    let cr_alt_acc = alt_trns
        .first()
        .cloned()
        .and_then(|x| x.get_object_id("account").ok());
    let cr_alt: Option<(Thing, String)> = if let Some(cr) = cr_alt_acc {
        let acc = accounts
            .iter()
            .find(|x| x.get_object_id("_id").unwrap() == cr)
            .unwrap();
        if let Some(default_name) = acc.get_string("defaultName") {
            Some((
                ("account".to_string(), default_name.to_lowercase()).into(),
                acc.get_string("displayName").unwrap(),
            ))
        } else {
            Some((
                ("account".to_string(), cr.to_hex()).into(),
                acc.get_string("displayName").unwrap(),
            ))
        }
    } else {
        None
    };
    alt_trns.sort_by(|a, b| {
        b._get_f64("credit")
            .unwrap_or_default()
            .total_cmp(&a._get_f64("credit").unwrap_or_default())
    });
    let dr_alt_acc = alt_trns
        .first()
        .cloned()
        .and_then(|x| x.get_object_id("account").ok());
    let dr_alt: Option<(Thing, String)> = if let Some(dr) = dr_alt_acc {
        let acc = accounts
            .iter()
            .find(|x| x.get_object_id("_id").unwrap() == dr)
            .unwrap();
        if let Some(default_name) = acc.get_string("defaultName") {
            Some((
                ("account".to_string(), default_name.to_lowercase()).into(),
                acc.get_string("displayName").unwrap(),
            ))
        } else {
            Some((
                ("account".to_string(), dr.to_hex()).into(),
                acc.get_string("displayName").unwrap(),
            ))
        }
    } else {
        None
    };
    (cr_alt, dr_alt)
}

impl Voucher {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("voucher INDEX start");
        surrealdb
            .query("DEFINE INDEX br ON TABLE voucher COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX base_voucher ON TABLE voucher COLUMNS base_voucher_type")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("voucher INDEX end");
        println!("voucher download start");
        let acc_find_opts = FindOptions::builder()
            .projection(doc! {"displayName": 1, "defaultName": 1})
            .build();
        let accounts = mongodb
            .collection::<Document>("accounts")
            .find(doc! {}, acc_find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let v_types = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, None)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        for collection in [
            "payments",
            "receipts",
            "sales",
            "purchases",
            "credit_notes",
            "debit_notes",
            "journals",
            "contras",
        ] {
            println!("{} download start", collection);
            let find_opts = FindOptions::builder().limit(10).build();
            let mut cur = mongodb
                .collection::<Document>(collection)
                .find(doc! {}, find_opts)
                .await
                .unwrap();

            while let Some(Ok(d)) = cur.next().await {
                let mode = if ["sales", "credit_nodes", "debit_notes", "purchases"]
                    .contains(&collection)
                {
                    d.get_string("mode")
                } else {
                    None
                };
                let id = d.get_oid_to_thing("_id", "voucher").unwrap();
                let branch = d.get_oid_to_thing("branch", "branch").unwrap();
                let eff_date = d
                    .get_surreal_datetime_from_str("effDate")
                    .unwrap_or(d.get_surreal_datetime_from_str("date").unwrap());
                let date = d.get_surreal_datetime_from_str("date").unwrap();
                let ref_no = d.get_string("refNo");
                let branch_name = d.get_string("branchName").unwrap();
                let base_voucher_type: Thing = (
                    "voucher_type".to_string(),
                    d.get_string("voucherType")
                        .unwrap()
                        .to_string()
                        .to_lowercase(),
                )
                    .into();
                let act = d.get_bool("act").unwrap_or_default();
                let act_hide = d.get_bool("actHide").unwrap_or_default();
                let voucher_no = d.get_string("voucherNo").unwrap();
                let voucher_type = v_types
                    .iter()
                    .find_map(|x| {
                        (x.get_object_id("_id").unwrap()
                            == d.get_object_id("voucherTypeId").unwrap())
                        .then_some(
                            x.get_bool("default")
                                .unwrap_or_default()
                                .then_some(
                                    (
                                        "voucher_type".to_string(),
                                        x.get_string("voucherType").unwrap().to_lowercase(),
                                    )
                                        .into(),
                                )
                                .unwrap(),
                        )
                    })
                    .unwrap_or(d.get_oid_to_thing("voucherTypeId", "voucher_type").unwrap());
                let _created: Created = surrealdb
                    .create("voucher")
                    .content(Self {
                        id: id.clone(),
                        branch: branch.clone(),
                        eff_date,
                        date: date.clone(),
                        ref_no,
                        description: d.get_string("description"),
                        created: d.get_surreal_datetime("createdAt").unwrap(),
                        updated: d.get_surreal_datetime("updatedAt").unwrap(),
                        voucher_type,
                        base_voucher_type: base_voucher_type.clone(),
                        act,
                        act_hide,
                        mode,
                        voucher_no,
                        amount: d._get_f64("amount").unwrap_or_default(),
                    })
                    .await
                    .unwrap()
                    .first()
                    .cloned()
                    .unwrap();
                if let Some(ac_trns) = d.get_array_document("acTrns") {
                    let (cr_alt, dr_alt) = get_alt_accounts(&ac_trns, &accounts);
                    for ac_trn in ac_trns {
                        let credit = ac_trn._get_f64("credit").unwrap();
                        let alt_account = if credit > 0.0 {
                            cr_alt.clone()
                        } else {
                            dr_alt.clone()
                        };
                        let account_doc = accounts
                            .iter()
                            .find(|x| {
                                x.get_object_id("_id").unwrap()
                                    == ac_trn.get_object_id("account").unwrap()
                            })
                            .unwrap();
                        let (account, account_name): (Thing, String) =
                            if let Some(default_name) = account_doc.get_string("defaultName") {
                                (
                                    ("account".to_string(), default_name.to_lowercase()).into(),
                                    account_doc.get_string("displayName").unwrap(),
                                )
                            } else {
                                (
                                    ac_trn.get_oid_to_thing("account", "account").unwrap(),
                                    account_doc.get_string("displayName").unwrap(),
                                )
                            };
                        let _created: Created = surrealdb
                            .create("account_transaction")
                            .content(AccountTransaction {
                                date: date.clone(),
                                debit: ac_trn._get_f64("debit").unwrap(),
                                credit,
                                account,
                                account_name,
                                account_type: (
                                    "account_type".to_string(),
                                    ac_trn.get_string("accountType").unwrap().to_lowercase(),
                                )
                                    .into(),
                                branch: branch.clone(),
                                branch_name: branch_name.clone(),
                                act,
                                act_hide,
                                alt_account: alt_account.clone().map(|x| x.0),
                                alt_account_name: alt_account.map(|x| x.1),
                                ref_no: ac_trn.get_string("refNo"),
                                voucher_no: ac_trn.get_string("voucherNo"),
                                base_voucher_type: Some(base_voucher_type.clone()),
                                voucher: Some(id.clone()),
                                is_opening: ac_trn.get_bool("isOpening").ok(),
                                voucher_mode: ac_trn.get_string("voucherMode"),
                            })
                            .await
                            .unwrap()
                            .first()
                            .cloned()
                            .unwrap();
                    }
                }
            }
            println!("{} download end", collection);
        }
        println!("voucher download end");
    }
}
