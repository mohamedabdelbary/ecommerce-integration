#[cfg(test)]
mod test_postgres {
    use crate::postgres::{values_sql_lists, export_orders};
    use db::postgres::{run_query, create_pool};
    use db::schema::create;
    use deadpool_postgres::Pool;
    use graphql::entities::{Address, Order, Customer, MoneyAmount, CurrencyCode};

    const SCHEMA: &str = "test";

    fn get_db_pool() -> Pool {
        let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
        let db_port: u16 = 5432;
        let db_conn_pool: usize = 1;
        let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| String::from("ecommerce"));
        let db_user = std::env::var("DB_USER").unwrap();
        let db_pass = std::env::var("DB_PASS").unwrap_or_else(|_| String::from(""));
        create_pool(&db_host, &db_user, &db_pass, &db_name, db_port, db_conn_pool)
    }

    struct MockEntity {
        id: String,
        name: String
    }

    fn mock_entities_string_combine(e: &MockEntity) -> String {
        vec![e.id.as_ref(), e.name.as_ref()].join(",").to_string()
    }

    #[test]
    fn test_sql_values() {
        let l = vec![
            MockEntity{id: String::from("1"), name: String::from("Avon Barksdale")},
            MockEntity{id: String::from("2"), name: String::from("Jimmy Mcnulty")},
            MockEntity{id: String::from("3"), name: String::from("Omar Little")}
        ];
        let values = values_sql_lists::<MockEntity>(&l, &mock_entities_string_combine);
        assert_eq!(values, String::from("(1,Avon Barksdale),(2,Jimmy Mcnulty),(3,Omar Little)"))
    }

    async fn setup_test_schema(pool: &Pool) {
        create(&SCHEMA, &pool).await;
        delete_order_records(&pool).await;
    }

    async fn delete_order_records(pool: &Pool) {
        // In case table already exists
        let delete_from_orders = format!("DELETE FROM {}.orders", SCHEMA);
        run_query::<&str>(&delete_from_orders, vec![], &pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_insert_orders() {
        let pool = get_db_pool();
        setup_test_schema(&pool).await;
        let orders = vec![
            Order {
                name: String::from("foo"),
                customer: Customer { id: String::from("123") },
                created_at: String::from("2021-10-04 12:37:04+00"),
                updated_at: String::from("2021-10-04 12:42:04+00"),
                shipping_address: Address {
                    line_1: String::from(""),
                    line_2: String::from(""),
                    zip: String::from("")
                },
                fully_paid: true,
                can_mark_as_paid: false,
                current_total_price: MoneyAmount { amount: 34.0, currency: CurrencyCode::EGP },
                original_total_price: MoneyAmount { amount: 50.0, currency: CurrencyCode::EGP },
                total_refund:  MoneyAmount { amount: 0.0, currency: CurrencyCode::EGP }
            },

            Order {
                name: String::from("bar"),
                customer: Customer { id: String::from("567") },
                created_at: String::from("2021-10-11 10:12:19+00"),
                updated_at: String::from("2021-10-11 10:12:19+00"),
                shipping_address: Address {
                    line_1: String::from(""),
                    line_2: String::from(""),
                    zip: String::from("")
                },
                fully_paid: true,
                can_mark_as_paid: false,
                current_total_price: MoneyAmount { amount: 17.0, currency: CurrencyCode::EGP },
                original_total_price: MoneyAmount { amount: 43.0, currency: CurrencyCode::EGP },
                total_refund:  MoneyAmount { amount: 0.0, currency: CurrencyCode::EGP }
            }
        ];
        export_orders(&SCHEMA, &orders, &pool).await.unwrap();

        // Test counts
        let cnt_query = format!("SELECT count(*) as cnt FROM {}.orders", SCHEMA);
        let cnt_res: i64 = run_query::<&str>(&cnt_query, vec![], &pool).await.unwrap()[0].get("cnt");
        assert_eq!(cnt_res, 2);
        // Test names
        let name_query = format!("SELECT name FROM {}.orders order by name", SCHEMA);
        let res = run_query::<&str>(&name_query, vec![], &pool).await.unwrap();
        let mut names: Vec<String> = vec![];
        for row in res { names.push(row.get(0)); }
        assert_eq!(names, vec!["bar".to_string(), "foo".to_string()]);
        // To keep the table clean for subsequent test runs
        delete_order_records(&pool).await;
    }
}
