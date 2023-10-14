use super::{
    doc, serialize_opt_tax_as_thing, AmountInfo, Database, Doc, Document, GstInfo, InventoryCess,
    Serialize, StreamExt, Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::{bson::from_document, options::FindOptions};
const COLLECTIONS: [&str; 6] = [
    "payments",
    "contras",
    "receipts",
    "journals",
    "purchases",
    "credit_notes",
    // "debit_notes",
    // "sales",
];

fn get_alt_accounts(ac_trns: &[Document], accounts: &[Document]) -> (Option<Thing>, Option<Thing>) {
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
    let cr_alt: Option<Thing> = if let Some(cr) = cr_alt_acc {
        let acc = accounts
            .iter()
            .find(|x| x.get_object_id("_id").unwrap() == cr)
            .unwrap();
        if let Some(default_name) = acc.get_string("defaultName") {
            Some(("account".to_string(), default_name.to_lowercase()).into())
        } else {
            Some(("account".to_string(), cr.to_hex()).into())
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
    let dr_alt: Option<Thing> = if let Some(dr) = dr_alt_acc {
        let acc = accounts
            .iter()
            .find(|x| x.get_object_id("_id").unwrap() == dr)
            .unwrap();
        if let Some(default_name) = acc.get_string("defaultName") {
            Some(("account".to_string(), default_name.to_lowercase()).into())
        } else {
            Some(("account".to_string(), dr.to_hex()).into())
        }
    } else {
        None
    };
    (cr_alt, dr_alt)
}

#[derive(Debug, Serialize)]
pub struct BillAllocationApiInput {
    pub sno: usize,
    pub pending: String,
    pub ref_type: String,
    pub amount: f64,
    pub bill_date: String,
    pub ref_no: String,
}

#[derive(Debug, Serialize)]
pub struct VoucherAccTransactionApiInput {
    pub sno: usize,
    pub id: Thing,
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    pub is_default: Option<bool>,
    pub account: Thing,
    pub credit: f64,
    pub debit: f64,
    bill_allocations: Option<Vec<BillAllocationApiInput>>,
}
#[derive(Debug, Serialize)]
pub struct VoucherInvTransactionApiInput {
    pub sno: usize,
    pub id: Thing,
    pub inventory: Thing,
    pub unit_conv: f64,
    pub unit_precision: u8,
    pub rate: Option<f64>,
    pub cost: Option<f64>,
    pub qty: Option<f64>,
    pub free_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    pub s_inc: Option<Thing>,
    pub disc: Option<AmountInfo>,
    pub batch: Thing,
    pub cess: Option<InventoryCess>,
    pub tax_inc: Option<bool>,
    pub nlc: Option<f64>,
    pub taxable_amount: Option<f64>,
    pub cgst_amount: Option<f64>,
    pub sgst_amount: Option<f64>,
    pub igst_amount: Option<f64>,
    pub cess_amount: Option<f64>,
    pub asset_amount: Option<f64>,
    pub sale_taxable_amount: Option<f64>,
    pub sale_tax_amount: Option<f64>,
}
#[derive(Debug, Serialize)]
pub struct VoucherApiInput {
    pub id: Thing,
    pub date: String,
    pub voucher_no: String,
    pub eff_date: String,
    pub mode: Option<String>,
    pub ref_no: Option<String>,
    pub customer_group: Option<Thing>,
    pub patient: Option<Thing>,
    pub doctor: Option<Thing>,
    pub lut: Option<bool>,
    pub voucher_type: Thing,
    pub description: Option<String>,
    pub branch: Thing,
    pub contact: Option<Thing>,
    pub party_gst: Option<GstInfo>,
    pub branch_gst: Option<GstInfo>,
    pub particulars: Option<String>,
    pub amount: Option<f64>,
    pub ac_txns: Option<Vec<VoucherAccTransactionApiInput>>,
    pub inv_txns: Option<Vec<VoucherInvTransactionApiInput>>,
}

impl VoucherApiInput {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1})
            .build();
        let v_types = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let find_opts = FindOptions::builder().projection(
            doc! {"txnId": 1, "amount": 1, "pending": {"$toString": "$pending"}, "refType": 1,"effDate": 1, "refNo": 1, "voucherNo": 1 }
        ).build();
        let bill_allocs = mongodb
            .collection::<Document>("bill_allocations")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let find_opts = FindOptions::builder()
            .projection(doc! {"defaultName": 1})
            .build();
        let accounts = mongodb
            .collection::<Document>("accounts")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        for collection in COLLECTIONS {
            let find_opts = FindOptions::builder().limit(10).build();
            let mut cur = mongodb
                .collection::<Document>(collection)
                .find(doc! {}, find_opts)
                .await
                .unwrap();
            while let Some(Ok(d)) = cur.next().await {
                let voucher_type = v_types
                    .iter()
                    .find_map(|x| {
                        (x.get_object_id("_id").unwrap()
                            == d.get_object_id("voucherTypeId").unwrap()
                            && x.get_bool("default").unwrap_or_default())
                        .then_some(
                            (
                                "voucher_type".to_string(),
                                x.get_string("voucherType").unwrap().to_lowercase(),
                            )
                                .into(),
                        )
                    })
                    .unwrap_or(d.get_oid_to_thing("voucherTypeId", "voucher_type").unwrap());
                let contact = d
                    .get_oid_to_thing("vendor", "contact")
                    .or(d.get_oid_to_thing("customer", "contact"));
                let branch_gst = d._get_document("branchGst").map(|x| GstInfo {
                    reg_type: x.get_string("regType").unwrap(),
                    location: x.get_string("location"),
                    gst_no: x.get_string("gstNo"),
                });
                let party_gst = d._get_document("branchGst").map(|x| GstInfo {
                    reg_type: x.get_string("regType").unwrap(),
                    location: x.get_string("location"),
                    gst_no: x.get_string("gstNo"),
                });
                let mut ac_txns = Vec::new();
                if let Some(ac_trns) = d.get_array_document("acTrns") {
                    let (cr_alt, dr_alt) = get_alt_accounts(&ac_trns, &accounts);
                    for (sno, ac_trn) in ac_trns.iter().enumerate() {
                        let id = ac_trn.get_oid_to_thing("_id", "ac_txn").unwrap();
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
                        let account =
                            if let Some(default_name) = account_doc.get_string("defaultName") {
                                ("account".to_string(), default_name.to_lowercase()).into()
                            } else {
                                ac_trn.get_oid_to_thing("account", "account").unwrap()
                            };
                        let ba = bill_allocs.iter().filter(|x| {
                            x.get_object_id("txnId").unwrap()
                                == ac_trn.get_object_id("_id").unwrap()
                        });
                        let mut bill_allocations = Vec::new();
                        for (no, b) in ba.into_iter().enumerate() {
                            bill_allocations.push(BillAllocationApiInput {
                                sno: no + 1,
                                pending: b.get_string("pending").unwrap(),
                                ref_type: b.get_string("refType").unwrap().to_lowercase(),
                                amount: b._get_f64("amount").unwrap_or_default(),
                                bill_date: b.get_string("effDate").unwrap(),
                                ref_no: b
                                    .get_string("refNo")
                                    .unwrap_or(b.get_string("voucherNo").unwrap()),
                            })
                        }
                        ac_txns.push(VoucherAccTransactionApiInput {
                            sno: sno + 1,
                            id,
                            alt_account,
                            gst_tax: ac_trn.get_string("tax"),
                            is_default: ac_trn.get_bool("isDefault").ok(),
                            account,
                            credit,
                            debit: ac_trn._get_f64("debit").unwrap(),
                            bill_allocations: (!bill_allocations.is_empty())
                                .then_some(bill_allocations),
                        });
                    }
                }
                let mut inv_txns = Vec::new();
                if let Some(inv_trns) = d.get_array_document("invTrns") {
                    for (sno, inv_trn) in inv_trns.iter().enumerate() {
                        let nlc = if collection == "purchases" {
                            Some(
                                inv_trn._get_f64("taxableAmount").unwrap_or_default()
                                    / ((inv_trn._get_f64("qty").unwrap_or_default()
                                        + inv_trn._get_f64("freeQty").unwrap_or_default())
                                        * inv_trn._get_f64("unitConv").unwrap()),
                            )
                        } else {
                            None
                        };
                        inv_txns.push(VoucherInvTransactionApiInput {
                            sno: sno + 1,
                            id: inv_trn.get_oid_to_thing("_id", "inv_txn").unwrap(),
                            inventory: inv_trn.get_oid_to_thing("inventory", "inventory").unwrap(),
                            unit_conv: inv_trn._get_f64("unitConv").unwrap(),
                            unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                            rate: inv_trn._get_f64("rate"),
                            cost: None,
                            qty: inv_trn._get_f64("qty"),
                            free_qty: inv_trn._get_f64("freeQty"),
                            gst_tax: inv_trn.get_string("tax"),
                            s_inc: inv_trn.get_oid_to_thing("s_inc", "sale_incharge"),
                            disc: inv_trn._get_document("disc").and_then(|x| {
                                (!x.is_empty()).then_some(from_document::<AmountInfo>(x).unwrap())
                            }),
                            batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                            cess: inv_trn._get_document("cess").and_then(|x| {
                                (!x.is_empty())
                                    .then_some(from_document::<InventoryCess>(x).unwrap())
                            }),
                            tax_inc: inv_trn.get_bool("taxInc").ok(),
                            nlc,
                            taxable_amount: inv_trn._get_f64("taxableAmount"),
                            cgst_amount: inv_trn._get_f64("cgstAmount"),
                            sgst_amount: inv_trn._get_f64("sgstAmount"),
                            igst_amount: inv_trn._get_f64("igstAmount"),
                            cess_amount: inv_trn._get_f64("cessAmount"),
                            asset_amount: inv_trn._get_f64("assetAmount"),
                            sale_taxable_amount: inv_trn._get_f64("saleTaxableAmount"),
                            sale_tax_amount: inv_trn._get_f64("saleTaxAmount"),
                        });
                    }
                }
                let input_data = Self {
                    id: d.get_oid_to_thing("_id", "voucher").unwrap(),
                    date: d.get_string("date").unwrap(),
                    voucher_no: d.get_string("voucherNo").unwrap(),
                    eff_date: d
                        .get_string("effDate")
                        .unwrap_or(d.get_string("date").unwrap()),
                    mode: d
                        .get_string("mode")
                        .map(|ref x| x.chars().skip(0).take(3).collect()),
                    ref_no: d.get_string("refNo"),
                    customer_group: d.get_oid_to_thing("customerGroup", "customer_group"),
                    patient: d.get_oid_to_thing("patient", "patient"),
                    doctor: d.get_oid_to_thing("doctor", "doctor"),
                    lut: d.get_bool("lut").ok(),
                    voucher_type,
                    description: d.get_string("description"),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    contact,
                    party_gst,
                    branch_gst,
                    particulars: None,
                    amount: d._get_f64("amount"),
                    ac_txns: (!ac_txns.is_empty()).then_some(ac_txns),
                    inv_txns: (!inv_txns.is_empty()).then_some(inv_txns),
                };

                let _created: Thing = surrealdb
                    .query("fn::save_voucher_script($data)")
                    .bind(("data", input_data))
                    .await
                    .unwrap()
                    .take::<Option<Thing>>(0)
                    .unwrap()
                    .unwrap();
            }
            println!("{} download end", &collection);
        }
    }

    pub async fn create_stock_journal(
        surrealdb: &Surreal<SurrealClient>,
        mongodb: &Database,
        collection: &str,
    ) {
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1})
            .build();
        let v_types = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let accounts = mongodb
            .collection::<Document>("accounts")
            .find(
                doc! {"accountType": {"$in": ["STOCK", "BRANCH_TRANSFER"]}},
                None,
            )
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let mut cur = mongodb
            .collection::<Document>(collection)
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let voucher_type = v_types
                .iter()
                .find_map(|x| {
                    (x.get_object_id("_id").unwrap() == d.get_object_id("voucherTypeId").unwrap()
                        && x.get_bool("default").unwrap_or_default())
                    .then_some(
                        (
                            "voucher_type".to_string(),
                            x.get_string("voucherType").unwrap().to_lowercase(),
                        )
                            .into(),
                    )
                })
                .unwrap_or(d.get_oid_to_thing("voucherTypeId", "voucher_type").unwrap());
            let mut ac_txns = Vec::new();
            if let Some(ac_trns) = d.get_array_document("acTrns") {
                let (cr_alt, dr_alt) = get_alt_accounts(&ac_trns, &accounts);
                for (sno, ac_trn) in ac_trns.iter().enumerate() {
                    let id = ac_trn.get_oid_to_thing("_id", "ac_txn").unwrap();
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
                    let account = if let Some(default_name) = account_doc.get_string("defaultName")
                    {
                        ("account".to_string(), default_name.to_lowercase()).into()
                    } else {
                        ac_trn.get_oid_to_thing("account", "account").unwrap()
                    };

                    ac_txns.push(VoucherAccTransactionApiInput {
                        sno: sno + 1,
                        id,
                        alt_account,
                        gst_tax: None,
                        is_default: ac_trn.get_bool("isDefault").ok(),
                        account,
                        credit,
                        debit: ac_trn._get_f64("debit").unwrap(),
                        bill_allocations: None,
                    });
                }
            }
            let mut inv_txns = Vec::new();
            if let Some(inv_trns) = d.get_array_document("invTrns") {
                for (sno, inv_trn) in inv_trns.iter().enumerate() {
                    let nlc = if collection == "stock_transfers"
                        && d.get_string("transferType") == Some("TARGET".to_string())
                    {
                        Some(
                            inv_trn._get_f64("taxableAmount").unwrap_or_default()
                                / ((inv_trn._get_f64("qty").unwrap_or_default()
                                    + inv_trn._get_f64("freeQty").unwrap_or_default())
                                    * inv_trn._get_f64("unitConv").unwrap()),
                        )
                    } else {
                        None
                    };
                    let qty = if d.get_string("transferType") == Some("SOURCE".to_string()) {
                        inv_trn._get_f64("qty").map(|x| x.abs() * -1.0)
                    } else {
                        inv_trn._get_f64("qty")
                    };
                    inv_txns.push(VoucherInvTransactionApiInput {
                        sno: sno + 1,
                        id: inv_trn.get_oid_to_thing("_id", "inv_txn").unwrap(),
                        inventory: inv_trn.get_oid_to_thing("inventory", "inventory").unwrap(),
                        unit_conv: inv_trn._get_f64("unitConv").unwrap(),
                        unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                        rate: inv_trn._get_f64("rate"),
                        cost: inv_trn._get_f64("cost"),
                        qty,
                        free_qty: None,
                        gst_tax: None,
                        s_inc: None,
                        disc: None,
                        batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                        cess: None,
                        tax_inc: None,
                        nlc,
                        taxable_amount: None,
                        cgst_amount: None,
                        sgst_amount: None,
                        igst_amount: None,
                        cess_amount: None,
                        asset_amount: inv_trn._get_f64("assetAmount"),
                        sale_taxable_amount: None,
                        sale_tax_amount: None,
                    });
                }
            }
            let input_data = Self {
                id: d.get_oid_to_thing("_id", "voucher").unwrap(),
                date: d.get_string("date").unwrap(),
                voucher_no: d.get_string("voucherNo").unwrap(),
                eff_date: d
                    .get_string("effDate")
                    .unwrap_or(d.get_string("date").unwrap()),
                mode: Some("INV".to_string()),
                ref_no: d.get_string("refNo"),
                customer_group: None,
                patient: None,
                doctor: None,
                lut: None,
                voucher_type,
                description: d.get_string("description"),
                branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                contact: None,
                party_gst: None,
                branch_gst: None,
                particulars: None,
                amount: d._get_f64("amount"),
                ac_txns: (!ac_txns.is_empty()).then_some(ac_txns),
                inv_txns: (!inv_txns.is_empty()).then_some(inv_txns),
            };

            let _created: Thing = surrealdb
                .query("fn::save_voucher_script($data)")
                .bind(("data", input_data))
                .await
                .unwrap()
                .take::<Option<Thing>>(0)
                .unwrap()
                .unwrap();
        }
        println!("{} download end", &collection);
    }
}
