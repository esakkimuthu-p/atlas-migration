use super::{
    doc, Created, Database, Datetime, Doc, Document, Serialize, StreamExt, Surreal, SurrealClient,
    Thing,
};

#[derive(Debug, Serialize)]
pub struct GstRegistration {
    pub id: Thing,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub reg_type: String,
    pub state_code: String,
    pub gst_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_invoice_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_password: Option<String>,
    pub created: Datetime,
    pub updated: Datetime,
}

impl GstRegistration {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("gst_registration INDEX start");
        surrealdb
            .query("DEFINE INDEX gst_no ON TABLE gst_registration COLUMNS gst_no")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("gst_registration INDEX end");
        println!("gst_registration download start");
        let mut cur = mongodb
            .collection::<Document>("gst_registrations")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("gst_registration")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "gst_registration").unwrap(),
                    gst_no: d.get_string("gstNo").unwrap(),
                    state_code: d.get_string("stateCode").unwrap(),
                    reg_type: d.get_string("regType").unwrap(),
                    username: d.get_string("username"),
                    email: d.get_string("email"),
                    e_invoice_username: d.get_string("eInvoiceUsername"),
                    e_password: d.get_string("ePassword"),
                    created: d.get_surreal_datetime("createdAt").unwrap(),
                    updated: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("gst_registration download end");
    }
}