use crate::util::{chunks, wrap_with, dt_to_string};
use db::postgres::run_query;
use deadpool_postgres::Pool;
use deadpool_postgres::tokio_postgres::Error;
use chrono::{DateTime, FixedOffset, Datelike, Timelike};

use graphql::entities::{Order, InventoryLevel, Entity};

const DEFAULT_START_TS: &str = "2020-01-01 00:00:00+00";


pub async fn export_entities<T>
    (
        schema: &str,
        entities: &Vec<T>,
        entity_insert_sql: &dyn Fn(&str, &Vec<T>) -> String,
        pool: &Pool
    ) -> Result<(), Error>
    where
        T: Entity + Clone
{
    for chunk in chunks(&entities, 20) {
        let stmt = entity_insert_sql(&schema, &chunk);
        run_query::<&str>(&stmt, vec![], &pool).await.unwrap();
    }
    Ok(())
}

pub async fn export_orders(schema: &str, orders: &Vec<Order>, pool: &Pool) -> Result<(), Error> {
    export_entities::<Order>(schema, orders, &orders_insert_stmt, pool).await
}

pub async fn export_inventory_levels(schema: &str, inv_levels: &Vec<InventoryLevel>, pool: &Pool) -> Result<(), Error> {
    export_entities::<InventoryLevel>(schema, inv_levels, &inventory_level_insert_stmt, pool).await
}

pub async fn max_orders_ts(schema: &str, pool: &Pool) -> String {
    max_ts(schema, "orders", "created_at", pool).await
}

pub async fn max_inventory_ts(schema: &str, pool: &Pool) -> String {
    max_ts(schema, "inventory_level", "created_at", pool).await
}

async fn max_ts(schema: &str, table: &str, ts_col: &str, pool: &Pool) -> String {
    let query = format!("SELECT max({}) as max_updated_at from {}.{}", ts_col, schema, table);
    let res = &run_query::<&str>(&query, vec![], &pool).await.unwrap();
    let rec = &res[0];
    match rec.get(0) {
        None => String::from(DEFAULT_START_TS),
        Some(dt) => dt_to_string(&dt)
    }
}

fn orders_insert_stmt(schema: &str, orders: &Vec<Order>) -> String {
    format!(
        "INSERT INTO {}.orders (
            name,
            customer_id,
            created_at,
            updated_at,
            shipping_address_line_1,
            shipping_address_line_2,
            shipping_address_zip,
            fully_paid,
            can_mark_as_paid,
            current_total_price_amount,
            current_total_price_currency,
            original_total_price_amount,
            original_total_price_currency,
            total_refund_amount,
            total_refund_currency
          )
          VALUES {}",
        schema,
        values_sql_lists::<Order>(&orders, &order_sql_string)
    )
}

fn inventory_level_insert_stmt(schema: &str, inv_levels: &Vec<InventoryLevel>) -> String {
    format!(
        "INSERT INTO {}.inventory_level (
            item_id,
            display_name,
            location_id,
            price,
            currency,
            quantity,
            created_at
          )
          VALUES {}",
        schema,
        values_sql_lists::<InventoryLevel>(&inv_levels, &inventory_level_sql_string)
    )
}

pub fn values_sql_lists<T>(entites: &Vec<T>, row_mapper: &dyn Fn(&T) -> String) -> String {
    let mut values = String::from("");
    for (i, entity) in entites.iter().enumerate() {
        values += &vec![
            "(".to_string(),
            row_mapper(&entity),
            ")".to_string()].join("");
        if i < entites.len() - 1 {values += ","}
    }
    String::from(values)
}

fn order_sql_string(order: &Order) -> String {
    let single_quote = "'";
    vec![
        wrap_with(order.name.replace("'", "''").as_str(), single_quote),
        wrap_with(order.customer.id.as_str(), single_quote),
        wrap_with(order.created_at.as_str(), single_quote),
        wrap_with(order.updated_at.as_str(), single_quote),
        wrap_with(order.shipping_address.line_1.replace("'", "''").as_str(), single_quote),
        wrap_with(order.shipping_address.line_2.replace("'", "''").as_str(), single_quote),
        wrap_with(order.shipping_address.zip.replace("'", "''").as_str(), single_quote),
        wrap_with(order.fully_paid.to_string().as_str(), single_quote),
        wrap_with(order.can_mark_as_paid.to_string().as_str(), single_quote),
        wrap_with(order.current_total_price.amount.to_string().as_str(), single_quote),
        wrap_with(order.current_total_price.currency.to_string().as_str(), single_quote),
        wrap_with(order.original_total_price.amount.to_string().as_str(), single_quote),
        wrap_with(order.original_total_price.currency.to_string().as_str(), single_quote),
        wrap_with(order.total_refund.amount.to_string().as_str(), single_quote),
        wrap_with(order.total_refund.currency.to_string().as_str(), single_quote)
    ].join(",").to_string()
}

fn inventory_level_sql_string(inv: &InventoryLevel) -> String {
    let single_quote = "'";
    vec![
        wrap_with(inv.item.id.as_str(), single_quote),
        wrap_with(inv.item.display_name.as_str(), single_quote),
        wrap_with(inv.location.id.as_str(), single_quote),
        wrap_with(inv.item.price.amount.to_string().as_str(), single_quote),
        wrap_with(inv.item.price.currency.to_string().as_str(), single_quote),
        wrap_with(inv.item.quantity.to_string().as_str(), single_quote),
        wrap_with(inv.created_at.as_str(), single_quote),
    ].join(",").to_string()
}
