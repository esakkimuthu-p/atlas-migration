use clap::Parser;
use mongodb::{
    bson::{doc, DateTime},
    Client as MongoClient,
};
use std::time::Instant;

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

mod model;

use model::{
    duplicate_fix, fix_batch_ref, Account, AccountOpening, Batch, Branch, Contact, DesktopClient,
    DiscountCode, Doctor, FinancialYear, GstRegistration, Inventory, InventoryOpening,
    Manufacturer, Member, Patient, PharmaSalt, PosTerminal, PrintTemplate, Rack, SaleIncharge,
    Section, TdsNatureOfPayment, Unit, VendorBillMapping, VendorItemMapping, VoucherApiInput,
    VoucherNumbering, VoucherType,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// mongodb Organization cluster MONGO-URI.
    #[clap(short, long)]
    uri: String,

    /// surreal Organization HOST.
    #[clap(short, long)]
    surreal: String,

    /// from_date.
    #[clap(short, long)]
    from_date: String,

    /// to_date.
    #[clap(short, long)]
    to_date: String,

    /// master.
    #[clap(short, long)]
    master: Option<bool>,

    /// opening.
    #[clap(short, long)]
    opening: Option<bool>,

    /// timestamp.
    #[clap(short, long)]
    created_at: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let surrealdb = Surreal::new::<Ws>(args.surreal).await.unwrap();
    surrealdb
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
    surrealdb.use_ns("test").await.unwrap();
    surrealdb.use_db("test").await.unwrap();

    let mongodb = MongoClient::with_uri_str(args.uri)
        .await
        .unwrap()
        .default_database()
        .unwrap();
    println!("company name {:?}", mongodb.name());
    println!("default account fix start");
    Account::map(&mongodb).await;
    println!("fix_batch_ref start");
    fix_batch_ref(&mongodb).await;
    println!("fix_batch_ref end");
    if args.master.unwrap_or_default() {
        if args.created_at.is_none() {
            println!("duplicate_fix start");
            duplicate_fix(&mongodb).await;
        }
        let filter = args
            .created_at
            .map(|dt| doc! {"createdAt": DateTime::parse_rfc3339_str(dt).unwrap()});
        println!("Rack download start");
        Rack::create(&surrealdb, &mongodb, filter).await;
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
    }
    if args.opening.unwrap_or_default() {
        let now = Instant::now();
        println!("AccountOpening download start");
        AccountOpening::set_account_opening(&surrealdb, &mongodb).await;
        println!(
            "AccountOpening download end duration {} sec",
            now.elapsed().as_secs()
        );
        let now = Instant::now();
        println!("InventoryOpening download start");
        InventoryOpening::set_inventory_opening(&surrealdb, &mongodb).await;
        println!(
            "InventoryOpening download end duration {} sec",
            now.elapsed().as_secs()
        );
    }
    for collection in [
        "payments",
        "receipts",
        "contras",
        "journals",
        "purchases",
        "credit_notes",
    ] {
        println!("{} download start", collection);
        let now = Instant::now();
        VoucherApiInput::create(
            &surrealdb,
            &mongodb,
            collection,
            doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
        )
        .await;
        println!(
            "{} download end duration {} sec",
            collection,
            now.elapsed().as_secs()
        );
    }
    println!("stock_transfers target download start");
    let now = Instant::now();
    VoucherApiInput::create_stock_journal(
        &surrealdb,
        &mongodb,
        "stock_transfers",
        doc! {"transferType": "TARGET", "date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "stock_transfers transferType TARGET download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("debit_notes download start");
    let now = Instant::now();
    VoucherApiInput::create(
        &surrealdb,
        &mongodb,
        "debit_notes",
        doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "debit_notes download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("stock_adjustments download start");
    let now = Instant::now();
    VoucherApiInput::create_stock_journal(
        &surrealdb,
        &mongodb,
        "stock_adjustments",
        doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "stock_adjustments download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("manufacturing_journals download start");
    let now = Instant::now();
    VoucherApiInput::create_stock_journal(
        &surrealdb,
        &mongodb,
        "manufacturing_journals",
        doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "manufacturing_journals download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("material_conversions download start");
    let now = Instant::now();
    VoucherApiInput::create_stock_journal(
        &surrealdb,
        &mongodb,
        "material_conversions",
        doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "material_conversions transferType SOURCE download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("stock_transfers download start");
    let now = Instant::now();
    VoucherApiInput::create_stock_journal(
        &surrealdb,
        &mongodb,
        "stock_transfers",
        doc! {"transferType": "SOURCE", "date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "stock_transfers transferType SOURCE download end duration {} sec",
        now.elapsed().as_secs()
    );
    println!("sales download start");
    let now = Instant::now();
    VoucherApiInput::create(
        &surrealdb,
        &mongodb,
        "sales",
        doc! {"date": {"$gte": &args.from_date, "$lte": &args.to_date }},
    )
    .await;
    println!(
        "sales download end duration {} sec",
        now.elapsed().as_secs()
    );
}
