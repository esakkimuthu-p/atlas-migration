use super::{
    doc, serialize_opt_tax_as_thing, Created, Database, Doc, Document, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};
use mongodb::options::FindOptions;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_favour_of: Option<String>,
    pub account_type: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_opt_tax_as_thing")]
    pub gst_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sac_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tds_nature_of_payment: Option<Thing>,
}

impl Account {
    pub async fn map(mongodb: &Database) {
        let updates = vec![
            doc! {"q": { "defaultName": {"$in": [
                "SALES",
                "SALES_NA",
                "SALES_EXEMPT",
                "SALES_NGS",
                "SALES_ZERO",
                "SALES_ZERO_POINT_ONE",
                "SALES_ZERO_POINT_TWO_FIVE",
                "SALES_ONE",
                "SALES_THREE",
                "SALES_ONE_POINT_FIVE",
                "SALES_TWENTY_EIGHT",
                "SALES_EIGHTEEN",
                "SALES_SEVEN_POINT_FIVE",
                "SALES_TWELVE",
                "SALES_FIVE",
            ]} }, "u": {"$set": {"surrealId": "sales"} }},
            doc! {"q": { "defaultName": {"$in": [
                "PURCHASES",
                "PURCHASES_NA",
                "PURCHASES_EXEMPT",
                "PURCHASES_NGS",
                "PURCHASES_ZERO",
                "PURCHASES_ZERO_POINT_ONE",
                "PURCHASES_ZERO_POINT_TWO_FIVE",
                "PURCHASES_ONE",
                "PURCHASES_THREE",
                "PURCHASES_ONE_POINT_FIVE",
                "PURCHASES_TWENTY_EIGHT",
                "PURCHASES_EIGHTEEN",
                "PURCHASES_SEVEN_POINT_FIVE",
                "PURCHASES_TWELVE",
                "PURCHASES_FIVE",
            ]} }, "u": {"$set": {"surrealId": "purchases"} }},
            doc! {"q": { "defaultName": {"$in": [
                "SGST_PAYABLE",
                "SGST_PAYABLE_ZERO_POINT_ZERO_FIVE",
                "SGST_PAYABLE_ZERO_POINT_ONE_TWO_FIVE",
                "SGST_PAYABLE_ONE_POINT_FIVE",
                "SGST_PAYABLE_ZERO_POINT_SEVEN_FIVE",
                "SGST_PAYABLE_ZERO_POINT_FIVE",
                "SGST_PAYABLE_TWO_POINT_FIVE",
                "SGST_PAYABLE_FOURTEEN",
                "SGST_PAYABLE_SIX",
                "SGST_PAYABLE_NINE",
                "SGST_PAYABLE_THREE_POINT_SEVEN_FIVE",
            ]} }, "u": {"$set": {"surrealId": "sgst_payable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "CGST_PAYABLE",
                "CGST_PAYABLE_ZERO_POINT_ZERO_FIVE",
                "CGST_PAYABLE_ZERO_POINT_FIVE",
                "CGST_PAYABLE_ZERO_POINT_ONE_TWO_FIVE",
                "CGST_PAYABLE_ZERO_POINT_SEVEN_FIVE",
                "CGST_PAYABLE_SIX",
                "CGST_PAYABLE_TWO_POINT_FIVE",
                "CGST_PAYABLE_NINE",
                "CGST_PAYABLE_FOURTEEN",
                "CGST_PAYABLE_THREE_POINT_SEVEN_FIVE",
                "CGST_PAYABLE_ONE_POINT_FIVE",
            ]} }, "u": {"$set": {"surrealId": "cgst_payable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "IGST_PAYABLE",
                "IGST_PAYABLE_ZERO_POINT_ONE",
                "IGST_PAYABLE_ZERO_POINT_TWO_FIVE",
                "IGST_PAYABLE_ONE",
                "IGST_PAYABLE_ONE_POINT_FIVE",
                "IGST_PAYABLE_THREE",
                "IGST_PAYABLE_FIVE",
                "IGST_PAYABLE_SEVEN_POINT_FIVE",
                "IGST_PAYABLE_TWELVE",
                "IGST_PAYABLE_EIGHTEEN",
                "IGST_PAYABLE_TWENTY_EIGHT",
            ]} }, "u": {"$set": {"surrealId": "igst_payable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "CESS_PAYABLE",
                "CESS_PAYABLE_NGS",
                "CESS_PAYABLE_NA",
                "CESS_PAYABLE_EXEMPT",
                "CESS_PAYABLE_ZERO",
                "CESS_PAYABLE_ZERO_POINT_ONE",
                "CESS_PAYABLE_ZERO_POINT_TWO_FIVE",
                "CESS_PAYABLE_ONE",
                "CESS_PAYABLE_ONE_POINT_FIVE",
                "CESS_PAYABLE_THREE",
                "CESS_PAYABLE_FIVE",
                "CESS_PAYABLE_SEVEN_POINT_FIVE",
                "CESS_PAYABLE_TWELVE",
                "CESS_PAYABLE_EIGHTEEN",
                "CESS_PAYABLE_TWENTY_EIGHT",
            ]} }, "u": {"$set": {"surrealId": "cess_payable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "SGST_RECEIVABLE",
                "SGST_RECEIVABLE_ZERO_POINT_ZERO_FIVE",
                "SGST_RECEIVABLE_ONE_POINT_FIVE",
                "SGST_RECEIVABLE_ZERO_POINT_ONE_TWO_FIVE",
                "SGST_RECEIVABLE_TWO_POINT_FIVE",
                "SGST_RECEIVABLE_ZERO_POINT_FIVE",
                "SGST_RECEIVABLE_ZERO_POINT_SEVEN_FIVE",
                "SGST_RECEIVABLE_SIX",
                "SGST_RECEIVABLE_NINE",
                "SGST_RECEIVABLE_THREE_POINT_SEVEN_FIVE",
                "SGST_RECEIVABLE_FOURTEEN",
            ]} }, "u": {"$set": {"surrealId": "sgst_receivable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "CGST_RECEIVABLE",
                "CGST_RECEIVABLE_ZERO_POINT_ZERO_FIVE",
                "CGST_RECEIVABLE_ZERO_POINT_FIVE",
                "CGST_RECEIVABLE_ZERO_POINT_ONE_TWO_FIVE",
                "CGST_RECEIVABLE_ZERO_POINT_SEVEN_FIVE",
                "CGST_RECEIVABLE_ONE_POINT_FIVE",
                "CGST_RECEIVABLE_TWO_POINT_FIVE",
                "CGST_RECEIVABLE_THREE_POINT_SEVEN_FIVE",
                "CGST_RECEIVABLE_SIX",
                "CGST_RECEIVABLE_FOURTEEN",
                "CGST_RECEIVABLE_NINE",
            ]} }, "u": {"$set": {"surrealId": "cgst_receivable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "IGST_RECEIVABLE",
                "IGST_RECEIVABLE_ZERO_POINT_ONE",
                "IGST_RECEIVABLE_ZERO_POINT_TWO_FIVE",
                "IGST_RECEIVABLE_ONE",
                "IGST_RECEIVABLE_ONE_POINT_FIVE",
                "IGST_RECEIVABLE_THREE",
                "IGST_RECEIVABLE_FIVE",
                "IGST_RECEIVABLE_SEVEN_POINT_FIVE",
                "IGST_RECEIVABLE_TWELVE",
                "IGST_RECEIVABLE_EIGHTEEN",
                "IGST_RECEIVABLE_TWENTY_EIGHT",
            ]} }, "u": {"$set": {"surrealId": "igst_receivable"} }},
            doc! {"q": { "defaultName": {"$in": [
                "CESS_RECEIVABLE",
                "CESS_RECEIVABLE_NGS",
                "CESS_RECEIVABLE_NA",
                "CESS_RECEIVABLE_EXEMPT",
                "CESS_RECEIVABLE_ZERO",
                "CESS_RECEIVABLE_ZERO_POINT_ONE",
                "CESS_RECEIVABLE_ZERO_POINT_TWO_FIVE",
                "CESS_RECEIVABLE_ONE",
                "CESS_RECEIVABLE_ONE_POINT_FIVE",
                "CESS_RECEIVABLE_THREE",
                "CESS_RECEIVABLE_FIVE",
                "CESS_RECEIVABLE_SEVEN_POINT_FIVE",
                "CESS_RECEIVABLE_TWELVE",
                "CESS_RECEIVABLE_EIGHTEEN",
                "CESS_RECEIVABLE_TWENTY_EIGHT",
            ]} }, "u": {"$set": {"surrealId": "cess_receivable"} }},
            doc! {"q": { "defaultName": "CASH"}, "u": {"$set": {"surrealId": "cash"}}},
        ];
        let command = doc! {
            "update": "accounts",
            "updates": &updates
        };
        mongodb.run_command(command, None).await.unwrap();
        println!("default account fix end");
    }

    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let find_opts = FindOptions::builder().sort(doc! {"_id": 1}).build();
        let mut cur = mongodb
            .collection::<Document>("accounts")
            .find(doc! {"surrealId": {"$exists": false}}, find_opts)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("account")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "account").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    alias_name: d.get_string("aliasName"),
                    in_favour_of: d.get_string("inFavourOf"),
                    account_type: (
                        "account_type".to_string(),
                        d.get_string("accountType").unwrap().to_lowercase(),
                    )
                        .into(),
                    gst_tax: d.get_string("tax"),
                    gst_type: d.get_string("gstType"),
                    sac_code: d.get_string("sacCode"),
                    parent: d.get_oid_to_thing("parentAccount", "account"),
                    description: d.get_string("description"),
                    tds_nature_of_payment: d
                        .get_oid_to_thing("tdsNatureOfPayment", "tds_nature_of_payment"),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("account download end");
    }
}
