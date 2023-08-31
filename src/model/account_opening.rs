use super::{
    doc, serialize_round_2, Created, Database, Datetime, Doc, Document, Serialize, StreamExt,
    Surreal, SurrealClient, Thing,
};

#[derive(Debug, Serialize)]
pub struct AccountOpening {
    pub id: Thing,
    pub account: Thing,
    pub branch: Thing,
    #[serde(serialize_with = "serialize_round_2")]
    pub credit: f64,
    #[serde(serialize_with = "serialize_round_2")]
    pub debit: f64,
    pub act_hide: bool,
    pub act: bool,
    pub updated_at: Datetime,
}

impl AccountOpening {
    pub async fn create(surrealdb: &Surreal<SurrealClient>, mongodb: &Database) {
        println!("account_opening INDEX start");
        surrealdb
            .query("DEFINE INDEX br ON TABLE account_opening COLUMNS branch")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        surrealdb
            .query("DEFINE INDEX acc ON TABLE account_opening COLUMNS account")
            .await
            .unwrap()
            .take::<Option<()>>(0)
            .unwrap();
        println!("account_opening INDEX end");
        println!("account_opening download start");
        let mut cur = mongodb
            .collection::<Document>("account_openings")
            .find(doc! {}, None)
            .await
            .unwrap();
        while let Some(Ok(d)) = cur.next().await {
            let _created: Created = surrealdb
                .create("account_opening")
                .content(Self {
                    id: d.get_oid_to_thing("_id", "account_opening").unwrap(),
                    account: d.get_oid_to_thing("account", "account").unwrap(),
                    branch: d.get_oid_to_thing("branch", "branch").unwrap(),
                    credit: d._get_f64("credit").unwrap_or_default(),
                    debit: d._get_f64("debit").unwrap_or_default(),
                    act_hide: d.get_bool("actHide").unwrap_or_default(),
                    act: d.get_bool("act").unwrap_or_default(),
                    updated_at: d.get_surreal_datetime("updatedAt").unwrap(),
                })
                .await
                .unwrap()
                .first()
                .cloned()
                .unwrap();
        }
        println!("account_opening download end");
    }
}
