use super::{
    doc, AddressInfo, ContactInfo, Created, Database, Doc, Document, GstInfo, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct Contact {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    pub contact_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_info: Option<GstInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<ContactInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aadhar_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_account: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tds_deductee_type: Option<Thing>,
}

impl Contact {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("contact INDEX start");
        surrealdb
            .query("DEFINE INDEX val_name ON TABLE contact COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX mob ON TABLE contact COLUMNS contact_info.mobile")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("contact INDEX end");
        println!("contact download start");
        let mut cur = mongodb
            .collection::<Document>("contacts")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let gst_info = d._get_document("gstInfo").map(|x| GstInfo {
                reg_type: x.get_string("regType").unwrap(),
                location: x.get_string("location"),
                gst_no: x.get_string("gstNo"),
            });
            let contact_info = d._get_document("contactInfo").map(|x| ContactInfo {
                mobile: x.get_string("mobile"),
                alternate_mobile: x.get_string("alternateMobile"),
                email: x.get_string("email"),
                telephone: x.get_string("telephone"),
                contact_person: x.get_string("contactPerson"),
            });
            let address_info = d._get_document("addressInfo").map(|x| AddressInfo {
                mobile: x.get_string("mobile"),
                city: x.get_string("city"),
                state: x
                    .get_string("state")
                    .map(|y| ("country".to_string(), y.to_lowercase()).into()),
                country: x
                    .get_string("country")
                    .map(|y| ("country".to_string(), y.to_lowercase()).into()),
                address: x.get_string("address"),
                pincode: x.get_string("pincode"),
            });
            let _created: Created = surrealdb
                .create("contact")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "contact").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    contact_type: d.get_string("contactType").unwrap(),
                    short_name: d.get_string("shortName"),
                    gst_info,
                    contact_info,
                    address_info,
                    aadhar_no: d.get_string("aadharNo"),
                    pan_no: d.get_string("panNo"),
                    credit_account: d.get_oid_to_thing("creditAccount", "account"),
                    tds_deductee_type: d
                        .get_string("tdsDeducteeType")
                        .map(|x| ("tds_deductee_type".to_string(), x.to_lowercase()).into()),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("contact download end");
    }
}
