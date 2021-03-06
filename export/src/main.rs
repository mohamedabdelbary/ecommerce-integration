use tokio;
use graphql::client::{Credentials, get_client};
use graphql::fetchers::{fetch_orders, fetch_inventory};
use db::postgres::create_pool;
use db::schema::create as create_schema;
use export::postgres::{export_orders, export_inventory_levels, max_orders_ts, max_inventory_ts};

#[tokio::main]
async fn main() {
    let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let db_port: u16 = std::env::var("DB_PORT").unwrap_or_else(|_| String::from("5432")).parse().unwrap();
    let db_conn_pool: usize = std::env::var("DB_CONN_POOL").unwrap_or_else(|_| String::from("16")).parse().unwrap();
    let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| String::from("ecommerce"));
    let db_schema = std::env::var("DB_SCHEMA").unwrap_or_else(|_| String::from("shopify"));
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap_or_else(|_| String::from(""));
    let pool = create_pool(&db_host, &db_user, &db_pass, &db_name, db_port, db_conn_pool);
    create_schema(&db_schema, &pool).await;

    let gql_host = std::env::var("GQL_HOST").expect("Must provide GraphQL host");
    let gql_user = std::env::var("GQL_API_USER").expect("Must provide GraphQL API user");
    let gql_pass = std::env::var("GQL_API_PASS").expect("Must provide GraphQL API password");
    let gql_creds = Credentials::new( gql_user, gql_pass );
    let client = get_client(&gql_creds);
    let max_order_ts = max_orders_ts(&db_schema, &pool).await;
    println!("Starting GraphQL Order fetch from {}", &max_order_ts);
    let orders = fetch_orders(&gql_host, &client, &max_order_ts).await.unwrap();
    println!("Fetched {} orders from GraphQL, starting export to DB", orders.len());
    export_orders(&db_schema, &orders, &pool).await.unwrap();
    println!("Exported orders to PG..");
    println!("====================");

    let max_inv_ts = max_inventory_ts(&db_schema, &pool).await;
    println!("Starting GraphQL Inventory fetch from {}", &max_inv_ts);
    let inv_levels = fetch_inventory(&gql_host, &client, &max_inv_ts).await.unwrap();
    println!("Fetched {} inventory levels from GraphQL, starting export to DB", inv_levels.len());
    export_inventory_levels(&db_schema, &inv_levels, &pool).await.unwrap();
    println!("Exported inventory levels to PG..");
    println!("Done!")
}
