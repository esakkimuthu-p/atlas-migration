use super::{
    doc, serialize_opt_tax_as_thing, AmountInfo, BillAllocationApiInput, Database, Doc, Document,
    GstInfo, Id, InventoryCess, Serialize, StreamExt, Surreal, SurrealClient, Thing, TryStreamExt,
};
use mongodb::{bson::from_document, options::FindOptions};

fn get_alt_accounts(
    ac_trns: &[Document],
    accounts: &[Document],
    inc_stock: bool,
) -> (Option<Thing>, Option<Thing>) {
    let mut alt_trns = ac_trns
        .iter()
        .cloned()
        .filter(|x| x.get_string("accountType").unwrap_or_default() != "STOCK" || inc_stock)
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
            .find(|x| x.get_object_id("_id").unwrap() == cr);
        if let Some(default_acc) = acc {
            Some(
                (
                    "account".to_string(),
                    default_acc
                        .get_string("defaultName")
                        .unwrap_or(cr.to_hex())
                        .to_lowercase(),
                )
                    .into(),
            )
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
            .find(|x| x.get_object_id("_id").unwrap() == dr);
        if let Some(default_acc) = acc {
            Some(
                (
                    "account".to_string(),
                    default_acc
                        .get_string("defaultName")
                        .unwrap_or(dr.to_hex())
                        .to_lowercase(),
                )
                    .into(),
            )
        } else {
            Some(("account".to_string(), dr.to_hex()).into())
        }
    } else {
        None
    };
    (cr_alt, dr_alt)
}

#[derive(Debug, Serialize)]
pub struct VoucherAccTransactionApiInput {
    pub sno: usize,
    pub id: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    pub account: Thing,
    pub credit: f64,
    pub debit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    bill_allocations: Option<Vec<BillAllocationApiInput>>,
}
#[derive(Debug, Serialize)]
pub struct VoucherInvTransactionApiInput {
    pub sno: usize,
    pub id: Thing,
    pub inventory: Thing,
    pub batch: Thing,
    pub unit_conv: f64,
    pub inward: f64,
    pub outward: f64,
    pub unit_precision: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_inc: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc: Option<AmountInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cess: Option<InventoryCess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igst_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cess_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_taxable_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_tax_amount: Option<f64>,
}
#[derive(Debug, Serialize)]
pub struct VoucherApiInput {
    pub id: Thing,
    pub date: String,
    pub voucher_no: String,
    pub voucher_prefix: String,
    pub voucher_fy: u16,
    pub voucher_seq: u32,
    pub eff_date: String,
    pub branch: Thing,
    pub voucher_type: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_group: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doctor: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lut: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_gst: Option<GstInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_gst: Option<GstInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particulars: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_txns: Option<Vec<VoucherAccTransactionApiInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_txns: Option<Vec<VoucherInvTransactionApiInput>>,
}

impl VoucherApiInput {
    fn voucher_no(v_no: &str, branch_prefix: &str, fy: &str) -> (String, u16, u32) {
        let mut alpha = v_no.split(char::is_numeric).collect::<String>();
        let numeric = v_no.split(char::is_alphabetic).collect::<String>();
        let mut seq = numeric.clone().split(fy).collect::<String>();
        if seq.is_empty() {
            seq = numeric;
        }
        if alpha.is_empty() {
            alpha = branch_prefix.to_string();
        }
        (
            alpha.clone(),
            fy.parse::<u16>().unwrap(),
            seq.parse::<u32>().unwrap(),
        )
    }

    pub async fn create(
        surrealdb: &Surreal<SurrealClient>,
        mongodb: &Database,
        collection: &str,
        filter: Document,
    ) {
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1, "prefix": 1})
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
            doc! {"txnId": 1, "amount": 1, "pending": {"$toString": "$pending"}, "refType": {"$toLower": "$refType"},"effDate": 1, "refNo": 1, "voucherNo": 1 }
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
            .find(doc! {"defaultName": {"$exists": true}}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let find_opts = FindOptions::builder()
            .projection(doc! {"voucherNoPrefix": 1})
            .build();
        let branches = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let fys = mongodb
            .collection::<Document>("financial_years")
            .find(doc! {}, None)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>(collection)
            .find(filter, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            if !d.get_bool("act").unwrap_or_default() {
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
                let party_gst = d._get_document("partyGst").map(|x| GstInfo {
                    reg_type: x.get_string("regType").unwrap(),
                    location: x.get_string("location"),
                    gst_no: x.get_string("gstNo"),
                });
                let mut ac_txns = Vec::new();
                let mut particulars = None;
                if let Some(ac_trns) = d.get_array_document("acTrns") {
                    let (cr_alt, dr_alt) = get_alt_accounts(&ac_trns, &accounts, false);
                    if ["contras", "debit_notes", "payments", "sales"].contains(&collection) {
                        particulars = cr_alt.clone();
                    } else {
                        particulars = dr_alt.clone();
                    }
                    for (sno, ac_trn) in ac_trns.iter().enumerate() {
                        let id = ac_trn.get_oid_to_thing("_id", "ac_txn").unwrap();
                        let credit = ac_trn._get_f64("credit").unwrap();
                        let alt_account = if credit > 0.0 {
                            cr_alt.clone()
                        } else {
                            dr_alt.clone()
                        };
                        let default_account_doc = accounts.iter().find(|x| {
                            x.get_object_id("_id").unwrap()
                                == ac_trn.get_object_id("account").unwrap()
                        });
                        let account = if let Some(default_acc) = default_account_doc {
                            (
                                "account".to_string(),
                                default_acc
                                    .get_string("defaultName")
                                    .unwrap()
                                    .to_lowercase(),
                            )
                                .into()
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
                        let (inward, outward) = match collection {
                            "purchases" => (
                                (inv_trn._get_f64("qty").unwrap_or_default()
                                    + inv_trn._get_f64("freeQty").unwrap_or_default())
                                    * inv_trn._get_f64("unitConv").unwrap(),
                                0.0,
                            ),
                            "debit_notes" => (
                                inv_trn._get_f64("qty").unwrap_or_default()
                                    * inv_trn._get_f64("unitConv").unwrap()
                                    * -1.0,
                                0.0,
                            ),
                            "sales" => (
                                0.0,
                                inv_trn._get_f64("qty").unwrap_or_default()
                                    * inv_trn._get_f64("unitConv").unwrap(),
                            ),
                            "credit_notes" => (
                                0.0,
                                inv_trn._get_f64("qty").unwrap_or_default()
                                    * inv_trn._get_f64("unitConv").unwrap()
                                    * -1.0,
                            ),
                            _ => panic!("invalid voucher"),
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
                            inward,
                            outward,
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
                let branch_prefix = branches
                    .iter()
                    .find_map(|x| {
                        (x.get_object_id("_id").unwrap() == d.get_object_id("branch").unwrap())
                            .then_some(x.get_str("voucherNoPrefix").unwrap())
                    })
                    .unwrap();
                let fy = fys
                    .iter()
                    .find(|x| {
                        x.get_string("fStart").unwrap() <= d.get_string("date").unwrap()
                            && x.get_string("fEnd").unwrap() >= d.get_string("date").unwrap()
                    })
                    .unwrap();
                let fy = format!(
                    "{}{}",
                    &fy.get_string("fStart").unwrap()[2..=3],
                    &fy.get_string("fEnd").unwrap()[2..=3]
                );

                let voucher_no =
                    Self::voucher_no(&d.get_string("voucherNo").unwrap(), branch_prefix, &fy);
                let input_data = Self {
                    id: d.get_oid_to_thing("_id", "voucher").unwrap(),
                    date: d.get_string("date").unwrap(),
                    voucher_no: d.get_string("voucherNo").unwrap(),
                    voucher_prefix: voucher_no.0,
                    voucher_fy: voucher_no.1,
                    voucher_seq: voucher_no.2,
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
                    particulars,
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
        }
        println!("{} download end", &collection);
    }

    pub async fn create_stock_journal(
        surrealdb: &Surreal<SurrealClient>,
        mongodb: &Database,
        collection: &str,
        filter: Document,
    ) {
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1, "prefix": 1})
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
        let find_opts = FindOptions::builder()
            .projection(doc! {"voucherNoPrefix": 1})
            .build();
        let branches = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let fys = mongodb
            .collection::<Document>("financial_years")
            .find(doc! {}, None)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>(collection)
            .find(filter, find_opts)
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
                let (cr_alt, dr_alt) = get_alt_accounts(&ac_trns, &accounts, true);
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
            match collection {
                "stock_adjustments" | "stock_transfers" => {
                    if let Some(inv_trns) = d.get_array_document("invTrns") {
                        for (sno, inv_trn) in inv_trns.iter().enumerate() {
                            let nlc = if collection == "stock_transfers"
                                && d.get_string("transferType") == Some("TARGET".to_string())
                            {
                                Some(
                                    inv_trn._get_f64("assetAmount").unwrap_or_default()
                                        / (inv_trn._get_f64("qty").unwrap_or_default()
                                            * inv_trn._get_f64("unitConv").unwrap()),
                                )
                            } else {
                                None
                            };
                            let qty = if d.get_string("transferType") == Some("SOURCE".to_string())
                            {
                                inv_trn._get_f64("qty").map(|x| x.abs() * -1.0)
                            } else {
                                inv_trn._get_f64("qty")
                            };
                            let (inward, outward) = if qty.unwrap_or_default() > 0.0 {
                                (
                                    qty.unwrap_or_default() * inv_trn._get_f64("unitConv").unwrap(),
                                    0.0,
                                )
                            } else {
                                (
                                    0.0,
                                    qty.unwrap_or_default() * inv_trn._get_f64("unitConv").unwrap(),
                                )
                            };

                            inv_txns.push(VoucherInvTransactionApiInput {
                                sno: sno + 1,
                                id: inv_trn.get_oid_to_thing("_id", "inv_txn").unwrap(),
                                inventory: inv_trn
                                    .get_oid_to_thing("inventory", "inventory")
                                    .unwrap(),
                                unit_conv: inv_trn._get_f64("unitConv").unwrap(),
                                unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                                rate: inv_trn._get_f64("rate"),
                                cost: inv_trn._get_f64("cost"),
                                qty,
                                inward,
                                outward,
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
                }
                "manufacturing_journals" => {
                    if let Some(inv_trn) = d._get_document("invTrn") {
                        let components = inv_trn.get_array_document("components").unwrap_or(vec![]);
                        let mut outter_cost = 0.0;
                        for (sno, comp) in components.iter().enumerate() {
                            outter_cost += inv_trn._get_f64("cost").unwrap_or_default()
                                / inv_trn._get_f64("unitConv").unwrap();
                            inv_txns.push(VoucherInvTransactionApiInput {
                                sno: sno + 1,
                                id: comp.get_oid_to_thing("_id", "inv_txn").unwrap(),
                                inventory: comp.get_oid_to_thing("inventory", "inventory").unwrap(),
                                unit_conv: comp._get_f64("unitConv").unwrap(),
                                unit_precision: comp._get_f64("unitPrecision").unwrap() as u8,
                                rate: None,
                                cost: comp._get_f64("cost"),
                                qty: comp._get_f64("qty").map(|x| x.abs() * -1.0),
                                free_qty: None,
                                gst_tax: None,
                                s_inc: None,
                                disc: None,
                                batch: comp.get_oid_to_thing("batch", "batch").unwrap(),
                                cess: None,
                                tax_inc: None,
                                nlc: None,
                                inward: 0.0,
                                outward: comp._get_f64("qty").unwrap_or_default().abs()
                                    * comp._get_f64("unitConv").unwrap(),
                                taxable_amount: None,
                                cgst_amount: None,
                                sgst_amount: None,
                                igst_amount: None,
                                cess_amount: None,
                                asset_amount: Some(
                                    comp._get_f64("cost").unwrap_or_default()
                                        * comp._get_f64("qty").unwrap_or_default(),
                                ),
                                sale_taxable_amount: None,
                                sale_tax_amount: None,
                            });
                        }
                        inv_txns.push(VoucherInvTransactionApiInput {
                            sno: components.len() + 1,
                            id: Thing {
                                id: Id::rand(),
                                tb: "inv_txn".to_string(),
                            },
                            inventory: inv_trn.get_oid_to_thing("inventory", "inventory").unwrap(),
                            unit_conv: inv_trn._get_f64("unitConv").unwrap(),
                            unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                            rate: None,
                            cost: Some(outter_cost * inv_trn._get_f64("unitConv").unwrap()),
                            qty: inv_trn._get_f64("qty").map(|x| x.abs()),
                            free_qty: None,
                            gst_tax: None,
                            s_inc: None,
                            disc: None,
                            inward: inv_trn
                                ._get_f64("qty")
                                .map(|x| x.abs() * inv_trn._get_f64("unitConv").unwrap())
                                .unwrap(),
                            outward: 0.0,
                            batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                            cess: None,
                            tax_inc: None,
                            nlc: Some(outter_cost / inv_trn._get_f64("unitConv").unwrap()),
                            taxable_amount: None,
                            cgst_amount: None,
                            sgst_amount: None,
                            igst_amount: None,
                            cess_amount: None,
                            asset_amount: Some(
                                outter_cost
                                    * inv_trn._get_f64("unitConv").unwrap()
                                    * inv_trn._get_f64("qty").unwrap_or_default(),
                            ),
                            sale_taxable_amount: None,
                            sale_tax_amount: None,
                        });
                    }
                }
                "material_conversions" => {
                    if let Some(inv_trns) = d.get_array_document("invTrns") {
                        let mut sr_no: usize = 0;
                        for inv_trn in inv_trns {
                            let mut source_qty: f64 = 0.0;
                            for target in inv_trn.get_array_document("targets").unwrap_or(vec![]) {
                                source_qty += target._get_f64("sourceQty").unwrap_or_default();
                                sr_no += 1;
                                inv_txns.push(VoucherInvTransactionApiInput {
                                    sno: sr_no,
                                    id: target.get_oid_to_thing("_id", "inv_txn").unwrap(),
                                    inventory: target
                                        .get_oid_to_thing("inventory", "inventory")
                                        .unwrap(),
                                    unit_conv: target._get_f64("unitConv").unwrap(),
                                    unit_precision: target._get_f64("unitPrecision").unwrap() as u8,
                                    rate: None,
                                    cost: target._get_f64("cost"),
                                    qty: target._get_f64("qty").map(|x| x.abs()),
                                    inward: target
                                        ._get_f64("qty")
                                        .map(|x| x.abs() * target._get_f64("unitConv").unwrap())
                                        .unwrap(),
                                    outward: 0.0,
                                    free_qty: None,
                                    gst_tax: None,
                                    s_inc: None,
                                    disc: None,
                                    batch: target.get_oid_to_thing("batch", "batch").unwrap(),
                                    cess: None,
                                    tax_inc: None,
                                    nlc: Some(
                                        target._get_f64("cost").unwrap_or_default()
                                            / target._get_f64("qty").unwrap_or_default(),
                                    ),
                                    taxable_amount: None,
                                    cgst_amount: None,
                                    sgst_amount: None,
                                    igst_amount: None,
                                    cess_amount: None,
                                    asset_amount: target._get_f64("assetAmount"),
                                    sale_taxable_amount: None,
                                    sale_tax_amount: None,
                                });
                            }
                            sr_no += 1;
                            inv_txns.push(VoucherInvTransactionApiInput {
                                sno: sr_no,
                                id: inv_trn.get_oid_to_thing("_id", "inv_txn").unwrap(),
                                inventory: inv_trn
                                    .get_oid_to_thing("inventory", "inventory")
                                    .unwrap(),
                                unit_conv: inv_trn._get_f64("unitConv").unwrap(),
                                unit_precision: inv_trn._get_f64("unitPrecision").unwrap() as u8,
                                rate: None,
                                cost: inv_trn._get_f64("cost"),
                                qty: Some(source_qty),
                                inward: 0.0,
                                outward: source_qty * inv_trn._get_f64("unitConv").unwrap(),
                                free_qty: None,
                                gst_tax: None,
                                s_inc: None,
                                disc: None,
                                batch: inv_trn.get_oid_to_thing("batch", "batch").unwrap(),
                                cess: None,
                                tax_inc: None,
                                nlc: None,
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
                }
                _ => panic!("internal err"),
            }
            let branch_prefix = branches
                .iter()
                .find_map(|x| {
                    (x.get_object_id("_id").unwrap() == d.get_object_id("branch").unwrap())
                        .then_some(x.get_str("voucherNoPrefix").unwrap())
                })
                .unwrap();
            let fy = fys
                .iter()
                .find(|x| {
                    x.get_string("fStart").unwrap() <= d.get_string("date").unwrap()
                        && x.get_string("fEnd").unwrap() >= d.get_string("date").unwrap()
                })
                .unwrap();
            let fy = format!(
                "{}{}",
                &fy.get_string("fStart").unwrap()[2..=3],
                &fy.get_string("fEnd").unwrap()[2..=3]
            );
            let voucher_no =
                Self::voucher_no(&d.get_string("voucherNo").unwrap(), branch_prefix, &fy);
            let input_data = Self {
                id: d.get_oid_to_thing("_id", "voucher").unwrap(),
                date: d.get_string("date").unwrap(),
                voucher_no: d.get_string("voucherNo").unwrap(),
                voucher_prefix: voucher_no.0,
                voucher_fy: voucher_no.1,
                voucher_seq: voucher_no.2,
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
