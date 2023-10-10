use super::{
    doc, Created, Database, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct Voucher {
    pub id: Thing,
    pub branch: Thing,
    pub voucher_type: Thing,
    pub base_voucher_type: Thing,
    pub date: String,
    pub eff_date: String,
    pub voucher_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: f64,
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
        surrealdb
            .query("DEFINE INDEX date ON TABLE voucher COLUMNS date")
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
        let inv_find_opts = FindOptions::builder()
            .projection(doc! { "displayName": 1, "sectionId": 1, "sectionName": 1, "manufacturerId":1, "manufacturerName": 1 })
            .build();
        let inventories = mongodb
            .collection::<Document>("inventories")
            .find(doc! {}, inv_find_opts)
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
            let find_opts = FindOptions::builder().limit(100).build();
            let mut cur = mongodb
                .collection::<Document>(collection)
                .find(doc! {}, find_opts)
                .await
                .unwrap();

            while let Some(Ok(d)) = cur.next().await {
                let (mode, contact, contact_name) = if [
                    "sales",
                    "credit_nodes",
                    "debit_notes",
                    "purchases",
                ]
                .contains(&collection)
                {
                    let contact = d
                        .get_oid_to_thing("vendor", "contact")
                        .or(d.get_oid_to_thing("customer", "contact"));
                    let contact_name = d.get_string("vendorName").or(d.get_string("customerName"));
                    (d.get_string("mode"), contact, contact_name)
                } else {
                    (None, None, None)
                };
                let id = d.get_oid_to_thing("_id", "voucher").unwrap();
                let branch = d.get_oid_to_thing("branch", "branch").unwrap();
                let eff_date = d
                    .get_string("effDate")
                    .unwrap_or(d.get_string("date").unwrap());
                let date = d.get_string("date").unwrap();
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
                        ref_no: ref_no.clone(),
                        contact: contact.clone(),
                        contact_name: contact_name.clone(),
                        voucher_type,
                        base_voucher_type: base_voucher_type.clone(),
                        mode: mode.clone().unwrap_or("ACC".to_string()),
                        voucher_no,
                        description: d.get_string("description"),
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
                        let id = ac_trn.get_oid_to_thing("_id", "ac_txn").unwrap();
                        let credit = ac_trn._get_f64("credit").unwrap();
                        let account_type = ac_trn.get_string("accountType").unwrap().to_lowercase();
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
                    }
                }
                if let Some(inv_trns) = d.get_array_document("invTrns") {
                    for inv_trn in inv_trns {
                        let qty = inv_trn._get_f64("qty");
                        let free_qty = inv_trn._get_f64("freeQty");
                        let unit_conv = inv_trn._get_f64("unitConv").unwrap();
                        let mut nlc = None;
                        let (inward, outward) = {
                            match collection {
                                "sales" => (0.0, qty.unwrap() * unit_conv),
                                "purchases" => {
                                    let inward = (qty.unwrap_or_default()
                                        + free_qty.unwrap_or_default())
                                        * unit_conv;
                                    nlc = Some(
                                        inv_trn._get_f64("taxableAmount").unwrap_or_default()
                                            / inward,
                                    );
                                    (inward, 0.0)
                                }
                                "credit_notes" => (0.0, -qty.unwrap() * unit_conv),
                                "debit_notes" => (-qty.unwrap() * unit_conv, 0.0),
                                _ => todo!(),
                            }
                        };
                        let inventory_doc = inventories
                            .iter()
                            .find(|x| {
                                x.get_object_id("_id").unwrap()
                                    == inv_trn.get_object_id("inventory").unwrap()
                            })
                            .unwrap();
                    }
                }
            }
            println!("{} download end", collection);
        }
        println!("voucher download end");
    }
}
