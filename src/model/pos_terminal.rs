use super::{
    doc, Created, Database, Doc, Document, HashSet, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

// fn default_sale() -> Thing {
//     ("voucher_type".to_string(), "sale".to_string()).into()
// }
// fn default_credit_note() -> Thing {
//     ("voucher_type".to_string(), "credit_note".to_string()).into()
// }
// fn default_payment() -> Thing {
//     ("voucher_type".to_string(), "payment".to_string()).into()
// }
// fn default_receipt() -> Thing {
//     ("voucher_type".to_string(), "receipt".to_string()).into()
// }
// fn default_contra() -> Thing {
//     ("voucher_type".to_string(), "contra".to_string()).into()
// }

#[derive(Debug, Serialize)]
pub struct PosSaleConfig {
    pub voucher_type: Thing,
    pub rate_editable: bool,
    pub discount_editable: bool,
    pub unit_editable: bool,
    pub auto_select_batch: bool,
    pub set_default_qty: bool,
    pub barcode_enabled: bool,
    pub cash_tender_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_voucher_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_template: Option<Thing>,
}

#[derive(Debug, Serialize)]
pub struct PosCreditNoteConfig {
    pub voucher_type: Thing,
    pub rate_editable: bool,
    pub discount_editable: bool,
    pub unit_editable: bool,
    pub auto_select_batch: bool,
    pub set_default_qty: bool,
    pub barcode_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_voucher_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_template: Option<Thing>,
}

#[derive(Debug, Serialize)]
pub struct PosContraConfig {
    pub voucher_type: Thing,
}

#[derive(Debug, Serialize)]
pub struct PosPaymentConfig {
    pub voucher_type: Thing,
    pub expense_only: bool,
}

#[derive(Debug, Serialize)]
pub struct PosReceiptConfig {
    // #[serde(default = "default_receipt")]
    pub voucher_type: Thing,
    pub income_only: bool,
}

#[derive(Debug, Serialize)]
pub struct PosGeneralConfig {
    pub offline_only: bool,
}

#[derive(Debug, Serialize)]
pub struct PosSettlementConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_template: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_denomination_difference: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct PosConfiguration {
    pub allowed_voucher_types: HashSet<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale: Option<PosSaleConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<PosCreditNoteConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contra: Option<PosContraConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PosPaymentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<PosReceiptConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<PosSettlementConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<PosGeneralConfig>,
}

#[derive(Debug, Serialize)]
pub struct PosTerminal {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub pass: String,
    pub branch: Thing,
    pub members: HashSet<Thing>,
    pub mode: String,
    // pub config: PosConfiguration,
    pub is_active: bool,
}

impl PosTerminal {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("pos_terminals")
            .find(doc! {}, None)
            .await
            .unwrap();
        let find_opts = FindOptions::builder()
            .projection(doc! {"default": 1, "voucherType": 1})
            .build();
        let _voucher_types = mongodb
            .collection::<Document>("voucher_types")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            // let mut sale: Option<PosSaleConfig> = None;
            // let mut credit_note: Option<PosCreditNoteConfig> = None;
            // let mut payment: Option<PosPaymentConfig> = None;
            // let mut receipt: Option<PosReceiptConfig> = None;
            // let mut contra: Option<PosContraConfig> = None;
            // let mut settlement: Option<PosSettlementConfig> = None;
            // let mut general: Option<PosGeneralConfig> = None;
            // let mut allowed_voucher_types = HashSet::new();
            // if let Some(config_doc) = d._get_document("configuration") {
            //     for (key, bs) in config_doc
            //         .iter()
            //         .map(|x| (x.0, x.1.as_document().unwrap_or(&doc! {}).clone()))
            //     {
            //         match key.as_str() {
            //             "sale" => {
            //                 let v_type = voucher_types.iter().find_map(|x| {
            //                     (x.get_object_id("_id").unwrap().to_hex()
            //                         == bs.get_string("voucherType").unwrap()
            //                         && x.get_bool("default").unwrap_or_default())
            //                     .then_some(("voucher_type".to_string(), "sale".to_string()).into())
            //                 });
            //                 sale = Some(PosSaleConfig {
            //                     voucher_type: v_type.unwrap_or(
            //                         (
            //                             "voucher_type".to_string(),
            //                             bs.get_string("voucherType").unwrap(),
            //                         )
            //                             .into(),
            //                     ),
            //                     rate_editable: bs.get_bool("rateEditable").unwrap_or_default(),
            //                     discount_editable: bs
            //                         .get_bool("discountEditable")
            //                         .unwrap_or_default(),
            //                     unit_editable: bs.get_bool("unitEditable").unwrap_or_default(),
            //                     auto_select_batch: bs
            //                         .get_bool("autoSelectBatch")
            //                         .unwrap_or_default(),
            //                     set_default_qty: bs.get_bool("setDefaultQty").unwrap_or_default(),
            //                     barcode_enabled: bs.get_bool("barcodeEnabled").unwrap_or_default(),
            //                     cash_tender_enabled: bs
            //                         .get_bool("cashTenderEnabled")
            //                         .unwrap_or_default(),
            //                     max_voucher_amount: bs._get_f64("maxVoucherAmount"),
            //                     print_template: bs
            //                         .get_string("printTemplate")
            //                         .map(|x| ("print_template".to_string(), x).into()),
            //                 })
            //             }
            //             "creditNote" => {
            //                 let v_type = voucher_types.iter().find_map(|x| {
            //                     (x.get_object_id("_id").unwrap().to_hex()
            //                         == bs.get_string("voucherType").unwrap()
            //                         && x.get_bool("default").unwrap_or_default())
            //                     .then_some(
            //                         ("voucher_type".to_string(), "credit_note".to_string()).into(),
            //                     )
            //                 });
            //                 credit_note = Some(PosCreditNoteConfig {
            //                     voucher_type: v_type.unwrap_or(
            //                         (
            //                             "voucher_type".to_string(),
            //                             bs.get_string("voucherType").unwrap(),
            //                         )
            //                             .into(),
            //                     ),
            //                     rate_editable: bs.get_bool("rateEditable").unwrap_or_default(),
            //                     discount_editable: bs
            //                         .get_bool("discountEditable")
            //                         .unwrap_or_default(),
            //                     unit_editable: bs.get_bool("unitEditable").unwrap_or_default(),
            //                     auto_select_batch: bs
            //                         .get_bool("autoSelectBatch")
            //                         .unwrap_or_default(),
            //                     set_default_qty: bs.get_bool("setDefaultQty").unwrap_or_default(),
            //                     barcode_enabled: bs.get_bool("barcodeEnabled").unwrap_or_default(),
            //                     max_voucher_amount: bs._get_f64("maxVoucherAmount"),
            //                     print_template: bs
            //                         .get_string("printTemplate")
            //                         .map(|x| ("print_template".to_string(), x).into()),
            //                 })
            //             }
            //             "payment" => {
            //                 let v_type = voucher_types.iter().find_map(|x| {
            //                     (x.get_object_id("_id").unwrap().to_hex()
            //                         == bs.get_string("voucherType").unwrap()
            //                         && x.get_bool("default").unwrap_or_default())
            //                     .then_some(
            //                         ("voucher_type".to_string(), "payment".to_string()).into(),
            //                     )
            //                 });
            //                 payment = Some(PosPaymentConfig {
            //                     voucher_type: v_type.unwrap_or(
            //                         (
            //                             "voucher_type".to_string(),
            //                             bs.get_string("voucherType").unwrap(),
            //                         )
            //                             .into(),
            //                     ),
            //                     expense_only: bs.get_bool("expenseOnly").unwrap_or_default(),
            //                 })
            //             }
            //             "receipt" => {
            //                 let v_type = voucher_types.iter().find_map(|x| {
            //                     (x.get_object_id("_id").unwrap().to_hex()
            //                         == bs.get_string("voucherType").unwrap()
            //                         && x.get_bool("default").unwrap_or_default())
            //                     .then_some(
            //                         ("voucher_type".to_string(), "receipt".to_string()).into(),
            //                     )
            //                 });
            //                 receipt = Some(PosReceiptConfig {
            //                     voucher_type: v_type.unwrap_or(
            //                         (
            //                             "voucher_type".to_string(),
            //                             bs.get_string("voucherType").unwrap(),
            //                         )
            //                             .into(),
            //                     ),
            //                     income_only: bs.get_bool("incomeOnly").unwrap_or_default(),
            //                 })
            //             }
            //             "contra" => {
            //                 let v_type = voucher_types.iter().find_map(|x| {
            //                     (x.get_object_id("_id").unwrap().to_hex()
            //                         == bs.get_string("voucherType").unwrap()
            //                         && x.get_bool("default").unwrap_or_default())
            //                     .then_some(
            //                         ("voucher_type".to_string(), "contra".to_string()).into(),
            //                     )
            //                 });
            //                 contra = Some(PosContraConfig {
            //                     voucher_type: v_type.unwrap_or(
            //                         (
            //                             "voucher_type".to_string(),
            //                             bs.get_string("voucherType").unwrap(),
            //                         )
            //                             .into(),
            //                     ),
            //                 })
            //             }
            //             "settlement" => {
            //                 settlement = Some(PosSettlementConfig {
            //                     print_template: bs
            //                         .get_string("printTemplate")
            //                         .map(|x| ("print_template".to_string(), x).into()),
            //                     max_denomination_difference: bs
            //                         ._get_f64("maxDenominationDifference"),
            //                 })
            //             }
            //             "allowedVoucherypes" => {
            //                 let x = config_doc
            //                     .get_array("allowedVoucherypes")
            //                     .unwrap_or_default().iter().map(|x|x.);
            //                 allowed_voucher_types = config_doc
            //                     .get_array_thing_from_str("allowedVoucherypes", "voucher_type")
            //                     .unwrap_or_default()
            //             }
            //             "general" => {
            //                 general = Some(PosGeneralConfig {
            //                     offline_only: bs.get_bool("offlineOnly").unwrap_or_default(),
            //                 })
            //             }
            //             _ => (),
            //         }
            //     }
            // }
            // let config = PosConfiguration {
            //     allowed_voucher_types,
            //     sale,
            //     credit_note,
            //     contra,
            //     payment,
            //     receipt,
            //     settlement,
            //     general,
            // };
            let _created: Created = surrealdb
                .create("pos_terminal")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "pos_terminal").unwrap(),
                    name: d.get_string("name").unwrap(),
                    pass: d.get_string("password").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    members: d.get_array_thing("members", "member").unwrap_or_default(),
                    mode: d.get_string("mode").unwrap(),
                    is_active: d.get_bool("isActive").unwrap_or_default(),
                    // config,
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("pos_terminal download end");
    }
}
