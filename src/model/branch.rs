use super::{
    doc, AddressInfo, ContactInfo, Created, Database, Doc, Document, HashSet, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

#[derive(Debug, Serialize)]
pub struct Branch {
    pub id: Thing,
    pub name: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<ContactInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
    pub voucher_no_prefix: String,
    pub members: HashSet<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<Document>,
    pub account: Thing,
    pub gst_registration: Thing,
}

impl Branch {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        let mut cur = mongodb
            .collection::<Document>("branches")
            .find(doc! {}, None)
            .await
            .unwrap();
        let find_opts = FindOptions::builder().projection(doc! {"gstNo": 1}).build();
        let gst_registrations = mongodb
            .collection::<Document>("gst_registrations")
            .find(doc! {}, find_opts)
            .await
            .unwrap()
            .try_collect::<Vec<Document>>()
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let gst_no = d
                ._get_document("gstInfo")
                .unwrap()
                .get_string("gstNo")
                .unwrap();
            let gst_registration = gst_registrations
                .iter()
                .find_map(|x| {
                    (x.get_string("gstNo").unwrap() == gst_no)
                        .then_some(x.get_oid_to_thing("_id", "gst_registration").unwrap())
                })
                .unwrap();
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
                .create("branch")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "branch").unwrap(),
                    name: d.get_string("name").unwrap(),
                    display_name: d.get_string("displayName").unwrap(),
                    contact_info,
                    address_info,
                    account: d.get_oid_to_thing("account", "account").unwrap(),
                    voucher_no_prefix: d.get_string("voucherNoPrefix").unwrap().to_uppercase(),
                    members: d.get_array_thing("members", "member").unwrap_or_default(),
                    misc: d.get_string("licenseNo").map(|x| doc! { "license_no": x }),
                    gst_registration,
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
