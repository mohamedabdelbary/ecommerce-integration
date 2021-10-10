use graphql_client::{GraphQLQuery, Response, QueryBody};
use std::error::Error;
use reqwest;
use std::fmt::Debug;
use crate::entities::{Order, Customer, Address, MoneyAmount, CurrencyCode};

type DateTime = String;
type Decimal = String;

// #[derive(GraphQLQuery)]
// #[graphql(
// schema_path = "schema.graphql",
// query_path = "queries/products.graphql",
// response_derives = "Debug,Serialize",
// )]
// pub struct ProductsQuery;
//
// #[derive(GraphQLQuery)]
// #[graphql(
// schema_path = "schema.graphql",
// query_path = "queries/customers.graphql",
// response_derives = "Debug,Serialize",
// )]
// pub struct CustomersQuery;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "schema.graphql",
query_path = "queries/orders.graphql",
response_derives = "Debug,Serialize",
)]
pub struct OrdersQuery;


async fn run_query<T: GraphQLQuery>(host: &str, client: &reqwest::Client, query: &QueryBody<T::Variables>) -> Result<Response<T::ResponseData>, Box<dyn Error>>
    where T::ResponseData: Debug
{
    let res = client.post(host).json(&query).send().await?;
    let response_body: Response<T::ResponseData> = res.json().await?;
    Ok(response_body)
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
    let mut orders = Vec::<Order>::new();
    let mut done = false;
    let mut start = start_ts.clone();
    while !done {
        let filter = format!("created_at:>'{}'", start);
        println!("Updated filter to {}", filter);
        let variables = orders_query::Variables {
            query_filter: filter
        };
        let query: QueryBody<orders_query::Variables> = OrdersQuery::build_query(variables);
        let resp: Response<orders_query::ResponseData> = run_query::<OrdersQuery>(&host, &client, &query).await.unwrap();
        let order_batch = extract_orders(resp);
        // FIXME: This doesn't work as expected because GQL server most likely throttles after
        //   a couple of requests and returns an error response, which extract_orders method would suppress
        //   and just return an empty order_batch.
        if order_batch.len() == 0 {
            println!("Finished processing at {}", start);
            done = true;
        } else {
            // orders are sorted by created_at so we know that latest timestamp will be that
            // of the last order in the batch.
            start = order_batch[order_batch.len() - 1].clone().created_at.to_string();
            println!("Length of order batch is {}", order_batch.len());
            println!("Updated start TS to {}", start);
            orders.extend(order_batch);
        }
    }
    Ok(orders)
}

// pub async fn get_max_date_orders(db_schema: &str, pool: &Pool) -> String {
//     // TODO: Continue
//     String::from("")
// }

fn extract_orders(gql_response: Response<orders_query::ResponseData>) -> Vec<Order> {
    let mut orders = Vec::<Order>::new();
    match gql_response.data {
        None => orders,
        Some(order_data) => {
            for o in order_data.orders.edges.iter() {
                let order = &o.node;
                let address = order.shipping_address.as_ref().unwrap();
                orders.push(Order {
                    name: order.name.to_string(),
                    customer: Customer {id: order.customer.as_ref().unwrap().id.to_string()},
                    created_at: order.created_at.to_string(),
                    updated_at: order.updated_at.to_string(),
                    shipping_address: Address {
                        line_1: address.address1.as_ref().unwrap().to_string(),
                        line_2: address.address2.as_ref().unwrap().to_string(),
                        zip: address.zip.as_ref().unwrap().to_string()
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
            orders
        }
    }

}

fn currency_map(curr: &orders_query::CurrencyCode) -> CurrencyCode {
    if *curr == orders_query::CurrencyCode::EGP {CurrencyCode::EGP}
    else if *curr == orders_query::CurrencyCode::GBP {CurrencyCode::GBP}
    else {CurrencyCode::USD}
}
