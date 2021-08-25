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
    println!("{:#?}", response_body);
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

pub async fn fetch_orders(host: &str, client: &reqwest::Client) -> Result<Vec<Order>, Box<dyn Error>> {
    // TODO: Pass start date and batch size, inject them as GQL variables and paginate through result set
    let variables = orders_query::Variables {};
    let query: QueryBody<orders_query::Variables> = OrdersQuery::build_query(variables);
    let resp: Response<orders_query::ResponseData> = run_query::<OrdersQuery>(&host, &client, &query).await.unwrap();
    let orders = extract_orders(resp);
    Ok(orders)
}

// pub async fn get_max_date_orders(db_schema: &str, pool: &Pool) -> String {
//     // TODO: Continue
//     String::from("")
// }

fn extract_orders(gql_response: Response<orders_query::ResponseData>) -> Vec<Order> {
    let mut orders = Vec::<Order>::new();
    for o in gql_response.data.unwrap().orders.edges.iter() {
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

fn currency_map(curr: &orders_query::CurrencyCode) -> CurrencyCode {
    if *curr == orders_query::CurrencyCode::EGP {CurrencyCode::EGP}
    else if *curr == orders_query::CurrencyCode::GBP {CurrencyCode::GBP}
    else {CurrencyCode::USD}
}
