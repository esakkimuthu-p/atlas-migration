use super::{
    doc, Created, Database, Datetime, Doc, Document, HashSet, Serialize, StreamExt, Surreal,
    SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct DateRule {
    pub name: Thing,
    pub today: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct Member {
    pub id: Thing,
    pub name: String,
    pub pass: String,
    pub remote_access: bool,
    pub is_act: bool,
    pub is_root: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub settings: Document,
}

impl Member {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("member INDEX start");
        surrealdb
            .query("DEFINE INDEX use ON TABLE member COLUMNS val_name")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("member INDEX end");
        println!("member download start");
        let mut cur = mongodb
            .collection::<Document>("members")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let mut id = d.get_oid_to_thing("_id", "member").unwrap();
            if d.get_bool("isRoot").unwrap_or_default() {
                id = ("member".to_string(), "admin".to_string()).into();
            }
            let _created: Created = surrealdb
                .create("member")
                .content(Self {
                    id,
                    name: d.get_string("username").unwrap(),
                    pass: d.get_string("password").unwrap(),
                    remote_access: d.get_bool("remoteAccess").unwrap_or_default(),
                    is_act: d.get_bool("isAccountant").unwrap_or_default(),
                    is_root: d.get_bool("isRoot").unwrap_or_default(),
                    user: d.get_object_id("user").ok().map(|x| x.to_hex()),
                    settings: d._get_document("settings").unwrap(),
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
