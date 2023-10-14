use mongodb::Client as MongoClient;

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

mod model;

use model::{
    duplicate_fix, Account, Batch, Branch, Contact, DesktopClient, DiscountCode, Doctor,
    FinancialYear, GstRegistration, Inventory, Manufacturer, Member, Patient, PharmaSalt,
    PosTerminal, PrintTemplate, Rack, SaleIncharge, Section, TdsNatureOfPayment, Unit,
    VoucherApiInput, VoucherNumbering, VoucherType,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let uri = std::env::var("URI").expect("URI must be set");
    let surrealdb = Surreal::new::<Ws>(db_host).await.unwrap();
    surrealdb
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
    surrealdb.use_ns("test").await.unwrap();
    surrealdb.use_db("test").await.unwrap();

    let mongodb = MongoClient::with_uri_str(uri)
        .await
        .unwrap()
        .default_database()
        .unwrap();
    duplicate_fix(&mongodb).await;
    println!("{:?}", mongodb.name());
    Rack::create(&surrealdb, &mongodb).await;
    PharmaSalt::create(&surrealdb, &mongodb).await;
    Unit::create(&surrealdb, &mongodb).await;
    Doctor::create(&surrealdb, &mongodb).await;
    Account::create(&surrealdb, &mongodb).await;
    Contact::create(&surrealdb, &mongodb).await;
    Section::create(&surrealdb, &mongodb).await;
    DesktopClient::create(&surrealdb, &mongodb).await;
    DiscountCode::create(&surrealdb, &mongodb).await;
    FinancialYear::create(&surrealdb, &mongodb).await;
    Manufacturer::create(&surrealdb, &mongodb).await;
    Patient::create(&surrealdb, &mongodb).await;
    VoucherType::create(&surrealdb, &mongodb).await;
    PosTerminal::create(&surrealdb, &mongodb).await;
    Member::create(&surrealdb, &mongodb).await;
    SaleIncharge::create(&surrealdb, &mongodb).await;
    GstRegistration::create(&surrealdb, &mongodb).await;
    PrintTemplate::create(&surrealdb, &mongodb).await;
    Branch::create(&surrealdb, &mongodb).await;
    Inventory::create(&surrealdb, &mongodb).await;
    Batch::create(&surrealdb, &mongodb).await;
    VoucherNumbering::create(&surrealdb, &mongodb).await;
    TdsNatureOfPayment::create(&surrealdb, &mongodb).await;
    VoucherApiInput::create(&surrealdb, &mongodb).await;
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "stock_transfers").await;
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "stock_adjustments").await;
}
