use serde_json::{json, Value};

use super::{
    doc, Created, Database, Doc, Document, HashSet, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};

#[derive(Debug, Serialize)]
pub struct VoucherType {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub base_type: Thing,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Thing>,
    pub config: Document,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<HashSet<Thing>>,
}

impl VoucherType {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("voucher_type INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE voucher_type COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("voucher_type INDEX end");
        println!("voucher_type download start");
        let mut cur = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let mut id = d.get_oid_to_thing("_id", "voucher_type").unwrap();
            if d.get_bool("default").unwrap_or_default() {
                id = (
                    "voucher_type".to_string(),
                    d.get_string("voucherType").unwrap().to_lowercase(),
                )
                    .into();
            }
            let config_doc = d
                ._get_document("config")
                .unwrap()
                ._get_document("c")
                .unwrap_or(doc! {});
            let configuration = match d.get_str("voucherType").unwrap_or_default() {
                "SALE" => {
                    let allowed_credit_accounts = config_doc
                        .get_array_thing("allowedCreditAccounts", "account")
                        .map(|x| {
                            x.into_iter()
                                .map(|y| y.to_string())
                                .collect::<Vec<String>>()
                        });
                    let config = doc! {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "warehouse_enabled" : config_doc.get_bool("warehouseEnabled").unwrap_or_default(),
                        "hide_rack" : config_doc.get_bool("hideRack").unwrap_or_default(),
                        "hide_mrp_in_batch_modal" : config_doc.get_bool("hideMrpInBatchModal").unwrap_or_default(),
                        "rate_editable" : config_doc.get_bool("rateEditable").unwrap_or_default(),
                        "tax_editable" : config_doc.get_bool("taxEditable").unwrap_or_default(),
                        "discount_editable" : config_doc.get_bool("discountEditable").unwrap_or_default(),
                        "unit_editable" : config_doc.get_bool("unitEditable").unwrap_or_default(),
                        "bill_discount_editable" : config_doc.get_bool("billDiscountEditable").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "set_focus_on_inventory" : config_doc.get_bool("setFocusOnInventory").unwrap_or_default(),
                        "auto_select_batch" : config_doc.get_bool("autoSelectBatch").unwrap_or_default(),
                        "set_default_qty" : config_doc.get_bool("setDefaultQty").unwrap_or_default(),
                        "allow_credit_customer" : config_doc.get_bool("allowCreditCustomer").unwrap_or_default(),
                        "enable_sale_incharge" : config_doc.get_bool("enableSaleIncharge").unwrap_or_default(),
                        "voucherwise_sale_incharge" : config_doc.get_bool("voucherwiseSaleIncharge").unwrap_or_default(),
                        "freeze_sale_incharge_for_voucher" : config_doc.get_bool("freezeSaleInchargeForVoucher").unwrap_or_default(),
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "customer_form_quick_create" : config_doc.get_bool("customerFormQuickCreate").unwrap_or_default(),
                        "pos_mode" : config_doc.get_bool("posMode").unwrap_or_default(),
                        "sale_mode" : config_doc.get_string("saleMode").unwrap_or("BOTH".to_string()),
                    };
                    // if let Some(x) = config_doc._get_document("inventory") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("inventory", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("account") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("account", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("gst") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("gst", inner_doc);
                    // }
                    doc! {"sale": config}
                }
                "CREDIT_NOTE" => {
                    let allowed_credit_accounts = config_doc
                        .get_array_thing("allowedCreditAccounts", "account")
                        .map(|x| {
                            x.into_iter()
                                .map(|y| y.to_string())
                                .collect::<Vec<String>>()
                        });
                    let config = doc! {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "warehouse_enabled" : config_doc.get_bool("warehouseEnabled").unwrap_or_default(),
                        "rate_editable" : config_doc.get_bool("rateEditable").unwrap_or_default(),
                        "tax_editable" : config_doc.get_bool("taxEditable").unwrap_or_default(),
                        "discount_editable" : config_doc.get_bool("discountEditable").unwrap_or_default(),
                        "unit_editable" : config_doc.get_bool("unitEditable").unwrap_or_default(),
                        "bill_discount_editable" : config_doc.get_bool("billDiscountEditable").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "allow_credit_customer" : config_doc.get_bool("allowCreditCustomer").unwrap_or_default(),
                        "enable_sale_incharge" : config_doc.get_bool("enableSaleIncharge").unwrap_or_default(),
                        "voucherwise_sale_incharge" : config_doc.get_bool("voucherwiseSaleIncharge").unwrap_or_default(),
                        "freeze_sale_incharge_for_voucher" : config_doc.get_bool("freezeSaleInchargeForVoucher").unwrap_or_default(),
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                        "customer_form_quick_create" : config_doc.get_bool("customerFormQuickCreate").unwrap_or_default(),
                        "pos_mode" : config_doc.get_bool("posMode").unwrap_or_default(),
                        "allowed_credit_accounts": allowed_credit_accounts.unwrap_or_default(),
                    };
                    // if let Some(x) = config_doc._get_document("inventory") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("inventory", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("account") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y);
                    //     }
                    //     config.insert("account", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("gst") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("gst", inner_doc);
                    // }
                    doc! {"credit_note": config}
                }
                "SALE_QUOTATION" => {
                    let config = doc! {
                        "rate_editable" : config_doc.get_bool("rateEditable").unwrap_or_default(),
                        "tax_editable" : config_doc.get_bool("taxEditable").unwrap_or_default(),
                        "discount_editable" : config_doc.get_bool("discountEditable").unwrap_or_default(),
                        "unit_editable" : config_doc.get_bool("unitEditable").unwrap_or_default(),
                        "bill_discount_editable" : config_doc.get_bool("billDiscountEditable").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                        "enable_silent_print_mode" : config_doc.get_bool("enableSilentPrintMode").unwrap_or_default(),
                    };
                    // if let Some(y) =
                    //     config_doc.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    // {
                    //     config.insert("default_print_template", y.to_string());
                    // }

                    doc! {"sale_quotation": config}
                }
                "PURCHASE" => {
                    let config = doc! {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "warehouse_enabled" : config_doc.get_bool("warehouseEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "s_rate_mrp_required" : config_doc.get_bool("sRateMrpRequired").unwrap_or_default(),
                        "s_rate_as_mrp" : config_doc.get_bool("sRateAsMrp").unwrap_or_default(),
                        "allow_credit_vendor" : config_doc.get_bool("allowCreditVendor").unwrap_or_default(),
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                    };
                    // if let Some(x) = config_doc._get_document("inventory") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("inventory", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("account") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("account", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("gst") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("gst", inner_doc);
                    // }
                    doc! {"purchase": config}
                }
                "DEBIT_NOTE" => {
                    let config = doc! {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "warehouse_enabled" : config_doc.get_bool("warehouseEnabled").unwrap_or_default(),
                        "rate_editable" : config_doc.get_bool("rateEditable").unwrap_or_default(),
                        "tax_editable" : config_doc.get_bool("taxEditable").unwrap_or_default(),
                        "discount_editable" : config_doc.get_bool("discountEditable").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "allow_credit_vendor" : config_doc.get_bool("allowCreditVendor").unwrap_or_default(),
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                    };
                    // if let Some(x) = config_doc._get_document("inventory") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("inventory", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("account") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("account", inner_doc);
                    // }
                    // if let Some(x) = config_doc._get_document("gst") {
                    //     let mut inner_doc = doc! {"enable_silent_print_mode": x.get_bool("enableSilentPrintMode").unwrap_or_default()};
                    //     if let Some(y) =
                    //         x.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    //     {
                    //         inner_doc.insert("default_print_template", y.to_string());
                    //     }
                    //     config.insert("gst", inner_doc);
                    // }
                    doc! {"debit_note": config}
                }
                "CONTRA" => {
                    doc! {"contra": {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                    }}
                }
                "PAYMENT" => {
                    doc! {"payment": {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "open_cheque_book_detail" : config_doc.get_bool("openChequeBookDetail").unwrap_or_default(),
                        "expense_only" : config_doc.get_bool("expenseOnly").unwrap_or_default(),
                    }}
                }
                "RECEIPT" => {
                    doc! {"receipt": {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "income_only" : config_doc.get_bool("incomeOnly").unwrap_or_default(),
                    }}
                }
                "JOURNAL" => {
                    doc! {"journal": {
                        "cash_register_enabled" : config_doc.get_bool("cashRegisterEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                    }}
                }
                "STOCK_ADJUSTMENT" => {
                    let mut config = doc! {
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "enable_silent_print_mode" : config_doc.get_bool("enableSilentPrintMode").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                    };
                    if let Some(y) =
                        config_doc.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    {
                        config.insert("default_print_template", y.to_string());
                    }
                    doc! {"stock_adjustment":config}
                }
                "STOCK_TRANSFER" => {
                    let config = doc! {
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "enable_silent_print_mode" : config_doc.get_bool("enableSilentPrintMode").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                    };
                    // if let Some(y) =
                    //     config_doc.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    // {
                    //     config.insert("default_print_template", y.to_string());
                    // }
                    doc! {"stock_transfer":config}
                }
                "MATERIAL_CONVERSION" => {
                    let config = doc! {
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                        "print_after_save" : config_doc.get_bool("printAfterSave").unwrap_or_default(),
                        "enable_silent_print_mode" : config_doc.get_bool("enableSilentPrintMode").unwrap_or_default(),
                        "enable_exp" : config_doc.get_bool("enableExp").unwrap_or_default(),
                    };
                    // if let Some(y) =
                    //     config_doc.get_oid_to_thing("defaultPrintTemplate", "print_template")
                    // {
                    //     config.insert("default_print_template", y.to_string());
                    // }
                    doc! {"material_conversion":config}
                }
                "MANUFACTURING_JOURNAL" => {
                    doc! {"manufacturing_journal": {
                        "barcode_enabled" : config_doc.get_bool("barcodeEnabled").unwrap_or_default(),
                    }}
                }
                _ => doc! {},
            };
            println!("{:?}, {:?}", configuration, id);
            let _created: Created = surrealdb
                .create("voucher_type")
                .content(Self {
                    id,
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    base_type: (
                        "voucher_type".to_string(),
                        d.get_string("voucherType").unwrap().to_lowercase(),
                    )
                        .into(),
                    is_default: d.get_bool("default").unwrap_or_default(),
                    prefix: d.get_string("prefix"),
                    sequence: d.get_oid_to_thing("seqId", "voucher_type"),
                    config: configuration,
                    members: d.get_array_thing("members", "member"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("member download end");
    }
}
