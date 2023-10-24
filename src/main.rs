use mongodb::Client as MongoClient;

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

mod model;

use model::{
    duplicate_fix, Account, AccountOpening, Batch, Branch, Contact, DesktopClient, DiscountCode,
    Doctor, FinancialYear, GstRegistration, Inventory, InventoryOpening, Manufacturer, Member,
    Patient, PharmaSalt, PosTerminal, PrintTemplate, Rack, SaleIncharge, Section,
    TdsNatureOfPayment, Unit, VendorBillMapping, VendorItemMapping, VoucherApiInput,
    VoucherNumbering, VoucherType,
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
    println!("Rack download start");
    Rack::create(&surrealdb, &mongodb).await;
    println!("PharmaSalt download start");
    PharmaSalt::create(&surrealdb, &mongodb).await;
    println!("Unit download start");
    Unit::create(&surrealdb, &mongodb).await;
    println!("Doctor download start");
    Doctor::create(&surrealdb, &mongodb).await;
    println!("account download start");
    Account::create(&surrealdb, &mongodb).await;
    println!("Contact download start");
    Contact::create(&surrealdb, &mongodb).await;
    println!("VendorItemMapping download start");
    VendorItemMapping::create(&surrealdb, &mongodb).await;
    println!("VendorBillMapping download start");
    VendorBillMapping::create(&surrealdb, &mongodb).await;
    println!("Section download start");
    Section::create(&surrealdb, &mongodb).await;
    println!("DesktopClient download start");
    DesktopClient::create(&surrealdb, &mongodb).await;
    println!("DiscountCode download start");
    DiscountCode::create(&surrealdb, &mongodb).await;
    println!("FinancialYear download start");
    FinancialYear::create(&surrealdb, &mongodb).await;
    println!("Manufacturer download start");
    Manufacturer::create(&surrealdb, &mongodb).await;
    println!("Patient download start");
    Patient::create(&surrealdb, &mongodb).await;
    println!("VoucherType download start");
    VoucherType::create(&surrealdb, &mongodb).await;
    println!("PosTerminal download start");
    PosTerminal::create(&surrealdb, &mongodb).await;
    println!("Member download start");
    Member::create(&surrealdb, &mongodb).await;
    println!("SaleIncharge download start");
    SaleIncharge::create(&surrealdb, &mongodb).await;
    println!("GstRegistration download start");
    GstRegistration::create(&surrealdb, &mongodb).await;
    println!("PrintTemplate download start");
    PrintTemplate::create(&surrealdb, &mongodb).await;
    println!("branch download start");
    Branch::create(&surrealdb, &mongodb).await;
    println!("Inventory download start");
    Inventory::create(&surrealdb, &mongodb).await;
    println!("Batch download start");
    Batch::create(&surrealdb, &mongodb).await;
    println!("VoucherNumbering download start");
    VoucherNumbering::create(&surrealdb, &mongodb).await;
    println!("TdsNatureOfPayment download start");
    TdsNatureOfPayment::create(&surrealdb, &mongodb).await;
    println!("AccountOpening download start");
    AccountOpening::set_account_opening(&surrealdb, &mongodb).await;
    println!("InventoryOpening download start");
    InventoryOpening::set_inventory_opening(&surrealdb, &mongodb).await;
    println!("purchases download start");
    VoucherApiInput::create(&surrealdb, &mongodb, "purchases").await;
    println!("credit_notes download start");
    VoucherApiInput::create(&surrealdb, &mongodb, "credit_notes").await;
    println!("debit_notes download start");
    VoucherApiInput::create(&surrealdb, &mongodb, "debit_notes").await;
    println!("sales download start");
    VoucherApiInput::create(&surrealdb, &mongodb, "sales").await;
    println!("stock_transfers download start");
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "stock_transfers").await;
    println!("stock_adjustments download start");
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "stock_adjustments").await;
    println!("manufacturing_journals download start");
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "manufacturing_journals").await;
    println!("material_conversions download start");
    VoucherApiInput::create_stock_journal(&surrealdb, &mongodb, "material_conversions").await;
}
