use mongodb::Client as MongoClient;

use surrealdb::{
    engine::remote::ws::Client as SurrealClient, engine::remote::ws::Ws, opt::auth::Root, Surreal,
};

mod model;

use model::{
    Account, AccountTransaction, Batch, DesktopClient, Doctor, Inventory, InventoryTransaction,
    PharmaSalt, Rack, Section, Unit,
};

pub static DB: Surreal<SurrealClient> = Surreal::init();

#[tokio::main]
async fn main() {
    DB.connect::<Ws>("localhost:8000")
        .await
        .expect("Error connecting to database");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    DB.use_ns("test").await.unwrap();
    DB.use_db("test").await.unwrap();

    let db = MongoClient::with_uri_str(
        "mongodb+srv://testadmin:rootroot@auditplus-test.dqqxs.mongodb.net/velavanmed",
    )
    .await
    .unwrap()
    .default_database()
    .unwrap();
    println!("{:?}", db.name());
    Account::create(&DB, &db).await;
    AccountTransaction::create(&DB, &db).await;
    Batch::create(&DB, &db).await;
    Rack::create(&DB, &db).await;
    InventoryTransaction::create(&DB, &db).await;
    Inventory::create(&DB, &db).await;
    Section::create(&DB, &db).await;
    Unit::create(&DB, &db).await;
    Doctor::create(&DB, &db).await;
    DesktopClient::create(&DB, &db).await;
    PharmaSalt::create(&DB, &db).await;
}
