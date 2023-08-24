use super::{
    doc, ContactInfo, Created, Database, Datetime, Doc, Document, GstInfo, HashSet, Serialize,
    StreamExt, Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct Branch {
    pub id: Thing,
    pub name: String,
    pub val_name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<ContactInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<Document>,
    pub voucher_no_prefix: String,
    pub members: HashSet<Thing>,
    pub gst_info: GstInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
    pub inventory_head: Thing,
    pub account: Thing,
    pub created: Datetime,
    pub updated: Datetime,
}

impl Branch {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("branch INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE branch COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("branch INDEX end");
        println!("branch download start");
        let mut cur = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let gst_info = d
                ._get_document("gstInfo")
                .map(|x| GstInfo {
                    reg_type: x.get_string("regType").unwrap(),
                    location: x.get_string("location"),
                    gst_no: x.get_string("gstNo"),
                })
                .unwrap();
            let contact_info = d._get_document("contactInfo").map(|x| ContactInfo {
                mobile: x.get_string("mobile"),
                alternate_mobile: x.get_string("alternateMobile"),
                email: x.get_string("email"),
                telephone: x.get_string("telephone"),
                contact_person: x.get_string("contactPerson"),
            });
            let _created: Created = surrealdb
                .create("branch")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "branch").unwrap(),
                    name: d.get_string("name").unwrap(),
                    val_name: d.get_string("validateName").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                    gst_info,
                    contact_info,
                    address_info: d._get_document("addressInfo"),
                    account: d.get_oid_to_thing("account", "account").unwrap(),
                    voucher_no_prefix: d.get_string("voucherNoPrefix").unwrap().to_uppercase(),
                    members: d.get_array_thing("members", "member").unwrap_or_default(),
                    misc: d
                        .get_string("licenseNo")
                        .map(|x| serde_json::json!({ "license_no": x })),
                    inventory_head: d
                        .get_oid_to_thing("inventoryHead", "inventory_head")
                        .unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("branch download end");
    }
}
