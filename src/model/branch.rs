use super::{doc, Database, Doc, Document, HashSet, PostgresClient, StreamExt};
use futures_util::TryStreamExt;
use mongodb::options::FindOptions;

pub struct Branch;

impl Branch {
    pub async fn create(postgres: &PostgresClient, mongodb: &Database) {
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
        let admin = mongodb
            .collection::<Document>("members")
            .find_one(doc! {"isRoot": true}, None)
            .await
            .unwrap()
            .unwrap()
            .get_object_id("_id")
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
                        .then_some(x.get_object_id("_id").unwrap().to_hex())
                })
                .unwrap();
            let contact_info = d._get_document("contactInfo").map(|x| {
                serde_json::json!({
                    "mobile": x.get_string("mobile"),
                    "alternate_mobile": x.get_string("alternateMobile"),
                    "email": x.get_string("email"),
                    "telephone": x.get_string("telephone"),
                    "contact_person": x.get_string("contactPerson"),
                })
            });
            let address_info = d._get_document("addressInfo").map(|x| {
                serde_json::json!({
                    "mobile": x.get_string("mobile"),
                    "city": x.get_string("city"),
                    "state": x
                        .get_string("state")
                        .map(|y| y.to_lowercase()),
                    "country": x
                        .get_string("country")
                        .map(|y| y.to_lowercase()),
                    "address": x.get_string("address"),
                    "pincode": x.get_string("pincode"),
                })
            });

            let mut members = HashSet::new();
            for item in d
                .get_array("members")
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|x| x.as_object_id())
            {
                if item != admin {
                    members.insert(item.to_hex());
                }
            }
            postgres
                .execute(
                    "INSERT INTO branch (id,name,display_name,contact_info,address_info,account,voucher_no_prefix,members,misc,gst_registration) 
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    &[
                        &d.get_object_id("_id").unwrap().to_hex(),
                        &d.get_string("name").unwrap(),
                        &d.get_string("displayName").unwrap(),
                        &contact_info,
                        &address_info,
                        &d.get_object_id("account").unwrap().to_hex(),
                        &d.get_string("voucherNoPrefix").unwrap(),
                        &members.into_iter().collect::<Vec<String>>(),
                        &d.get_string("licenseNo").map(|x| serde_json::json!({ "license_no": x })),
                        &gst_registration,
                    ],
                )
                .await
                .unwrap();
        }
        println!("branch download end");
    }
}
