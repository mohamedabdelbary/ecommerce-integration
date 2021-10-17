use crate::postgres::run_query;
use deadpool_postgres::Pool;

pub async fn create(schema_name: &str, pool: &Pool) {
    let schema_create = format!("CREATE SCHEMA IF NOT EXISTS {}", &schema_name);
    run_query::<&str>(&schema_create, vec![], &pool).await.unwrap();
    for stmt in table_statements(&schema_name).iter() {
        run_query::<&str>(stmt, vec![], &pool).await.unwrap();
    }
}

fn table_statements(schema_name: &str) -> Vec<String> {
    vec![
        orders(&schema_name),
        products(&schema_name),
        customers(&schema_name)
    ]
}

fn orders(schema_name: &str) -> String {
    format!("CREATE TABLE IF NOT EXISTS {}.orders (
        id                              serial primary key,
        name                            varchar(512),
        customer_id                     varchar(512),
        created_at                      timestamp,
        updated_at                      timestamp,
        shipping_address_line_1         text,
        shipping_address_line_2         text,
        shipping_address_zip            text,
        fully_paid                      bool,
        can_mark_as_paid                bool,
        current_total_price_amount      real,
        current_total_price_currency    varchar(5),
        original_total_price_amount     real,
        original_total_price_currency   varchar(5),
        total_refund_amount             real,
        total_refund_currency           varchar(5)
    )", schema_name)
}

fn inventory(schema_name: &str) -> String {
    // TODO: Complete
    String::from("")
}

fn products(schema_name: &str) -> String {
    // TODO: Complete
    String::from("")
}

fn customers(schema_name: &str) -> String {
    // TODO: Complete
    String::from("")
}

