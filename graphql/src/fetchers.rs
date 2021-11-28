use graphql_client::{GraphQLQuery, Response, QueryBody};
use std::error::Error;
use reqwest;
use std::fmt::Debug;
use std::{thread, time};
use crate::entities::{Order, Customer, Address, MoneyAmount, CurrencyCode, InventoryLevel, Entity};
use crate::error::GraphQLFetchError;

type DateTime = String;
type Decimal = String;
type Money = String;

// #[derive(GraphQLQuery)]
// #[graphql(
// schema_path = "schema.graphql",
// query_path = "queries/products.graphql",
// response_derives = "Debug,Serialize",
// )]
// pub struct ProductsQuery;
//

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "queries/orders.graphql",
response_derives = "Debug,Serialize",
)]
pub struct OrdersQuery;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "queries/inventory.graphql",
response_derives = "Debug,Serialize",
)]
pub struct InventoryQuery;


const RETRIES: u32 = 5;
const GQL_BATCH_SIZE: usize = 50;

async fn run_query<T: GraphQLQuery>(host: &str, client: &reqwest::Client, query: &QueryBody<T::Variables>) -> Result<Response<T::ResponseData>, Box<dyn Error>>
{
    let mut attempt: u32 = 1;
    let min_sleep_millis: u64 = 500;
    // Need to support retries as GQL server will fail if hit at a high rate
    while attempt <= RETRIES {
        let res = client.post(host).json(&query).send().await?;
        let response_body: Response<T::ResponseData> = res.json().await?;
        match response_body.errors {
            None => return Ok(response_body),
            Some(_) => {
                // Exponential Back-off
                let sleep_millis = min_sleep_millis * u64::pow(2, attempt);
                println!("GraphQL returned an error, sleeping {} seconds before retrying", (sleep_millis / 1000) as i32);
                thread::sleep(time::Duration::from_millis(sleep_millis));
                attempt += 1;
            }
        }
    }
    Err(Box::new(GraphQLFetchError()))
}

// pub async fn fetch_products(host: &str, client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
//     let resp = run_query(&host, &client, products_query);
//     // TODO: Parse products
// }
//
// pub async fn fetch_customers(host: &str, client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
//     let resp = run_query(&host, &client, customers_query);
//     // TODO: Parse customers
// }

pub async fn fetch_orders(host: &str, client: &reqwest::Client, start_ts: &String) -> Result<Vec<Order>, Box<dyn Error>> {
    fetch_from_gql::<Order, OrdersQuery>(&host, &client, &start_ts, &orders_gql_query, &extract_orders).await
}


fn orders_gql_query(filter: String, batch_size: i64) -> QueryBody<orders_query::Variables> {
    let variables = orders_query::Variables {
        query_filter: filter,
        batch_size: batch_size
    };
    OrdersQuery::build_query(variables)
}


// Generic function to run any GraphQL query with pagination and populate list of entities,
// starting from start_ts timestamp.
async fn fetch_from_gql<T, Q>
    (
        host: &str,
        client: &reqwest::Client,
        start_ts: &String, 
        gql_query_fn: &dyn Fn(String, i64) -> QueryBody<Q::Variables>,
        entity_extract_fn: &dyn for<'r> Fn(&'r Response<Q::ResponseData>) -> (Vec<T>, usize),
    ) -> Result<Vec<T>, Box<dyn Error>>
    where 
        T: Clone + Entity,
        Q: GraphQLQuery,
{
   
    let mut entities = Vec::<T>::new();
    let mut done = false;
    let mut start = start_ts.clone();
    while !done {
        let filter = format!("created_at:>'{}'", start);
        println!("Updated filter to {}", filter);
        let query: QueryBody<Q::Variables> = gql_query_fn(filter, GQL_BATCH_SIZE as i64);
        let resp: Response<Q::ResponseData> = run_query::<Q>(&host, &client, &query).await.unwrap();
        let (entity_batch, invalid_order_records) = entity_extract_fn(&resp);
        let batch_len = entity_batch.len() + invalid_order_records;
        println!("Length of batch is {}", batch_len);
        if batch_len < GQL_BATCH_SIZE {
            println!("Finished processing at {}", start);
            done = true;
        } else {
            start = entity_batch[entity_batch.len() - 1].clone().created_at().to_string();
            println!("Updated start timestamp to {}", start);
        }
        entities.extend(entity_batch);
    }
    Ok(entities)
}


pub fn extract_orders<'r>(gql_response: &'r Response<orders_query::ResponseData>) -> (Vec<Order>, usize) {
    let mut orders = Vec::<Order>::new();
    let mut invalid_order_records = 0;
    let empty_str = String::from("");
    match &gql_response.data {
        None => (orders, invalid_order_records),
        Some(order_data) => {
            for o in order_data.orders.edges.iter() {
                let order = &o.node;
                // No customer or address happens on very rare occasions, when it does, we will skip the order.
                if order.customer.as_ref().is_none() || order.shipping_address.as_ref().is_none() {
                    invalid_order_records += 1;
                    continue;
                }
                let address = order.shipping_address.as_ref().unwrap();
                orders.push(Order {
                    name: order.name.to_string(),
                    customer: Customer {id: order.customer.as_ref().unwrap().id.to_string()},
                    created_at: order.created_at.to_string(),
                    updated_at: order.updated_at.to_string(),
                    shipping_address: Address {
                        line_1: address.address1.as_ref().unwrap_or_else(|| &empty_str).to_string(),
                        line_2: address.address2.as_ref().unwrap_or_else(|| &empty_str).to_string(),
                        zip: address.zip.as_ref().unwrap_or_else(|| &empty_str).to_string()
                    },
                    fully_paid: order.fully_paid,
                    can_mark_as_paid: order.can_mark_as_paid,
                    current_total_price: MoneyAmount {
                        amount: order.current_total_price_set.shop_money.amount.parse::<f32>().unwrap(),
                        currency: currency_map(&order.current_total_price_set.shop_money.currency_code)
                    },
                    original_total_price: MoneyAmount {
                        amount: order.original_total_price_set.shop_money.amount.parse::<f32>().unwrap(),
                        currency: currency_map(&order.original_total_price_set.shop_money.currency_code)
                    },
                    total_refund:  MoneyAmount {
                        amount: order.total_refunded_set.shop_money.amount.parse::<f32>().unwrap(),
                        currency: currency_map(&order.total_refunded_set.shop_money.currency_code)
                    }
                })
            }
            (orders, invalid_order_records)
        }
    }
}

fn currency_map(curr: &orders_query::CurrencyCode) -> CurrencyCode {
    if *curr == orders_query::CurrencyCode::EGP {CurrencyCode::EGP}
    else if *curr == orders_query::CurrencyCode::GBP {CurrencyCode::GBP}
    else {CurrencyCode::USD}
}
