use mongodb::Client as MongoClient;

use surrealdb::{
    engine::remote::ws::Client as SurrealClient, engine::remote::ws::Ws, opt::auth::Root, Surreal,
};

mod model;

use model::{
    Account, AccountOpening, AccountTransaction, BankTransaction, Batch, BillAllocation, Branch,
    CashRegister, Contact, DesktopClient, DiscountCode, Doctor, FinancialYear, GstRegistration,
    Inventory, InventoryHead, InventoryOpening, InventoryTransaction, Manufacturer, Member,
    Patient, PharmaSalt, PosTerminal, PrintTemplate, Rack, SaleIncharge, Section,
    TdsNatureOfPayment, Unit, VendorBillMap, VendorItemMap, Voucher, VoucherNumbering, VoucherType,
};

pub static DB: Surreal<SurrealClient> = Surreal::init();

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let db_host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let uri = std::env::var("URI").expect("URI must be set");
    DB.connect::<Ws>(db_host)
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

    let db = MongoClient::with_uri_str(uri)
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
    DiscountCode::create(&DB, &db).await;
    BankTransaction::create(&DB, &db).await;
    Contact::create(&DB, &db).await;
    AccountOpening::create(&DB, &db).await;
    BillAllocation::create(&DB, &db).await;
    Branch::create(&DB, &db).await;
    FinancialYear::create(&DB, &db).await;
    VoucherNumbering::create(&DB, &db).await;
    CashRegister::create(&DB, &db).await;
    Manufacturer::create(&DB, &db).await;
    InventoryHead::create(&DB, &db).await;
    InventoryOpening::create(&DB, &db).await;
    Member::create(&DB, &db).await;
    Patient::create(&DB, &db).await;
    PosTerminal::create(&DB, &db).await;
    PrintTemplate::create(&DB, &db).await;
    VendorBillMap::create(&DB, &db).await;
    VendorItemMap::create(&DB, &db).await;
    SaleIncharge::create(&DB, &db).await;
    TdsNatureOfPayment::create(&DB, &db).await;
    VoucherType::create(&DB, &db).await;
    GstRegistration::create(&DB, &db).await;
    Voucher::create(&DB, &db).await;
}
