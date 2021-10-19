#[cfg(test)]
mod test_postgres {
    use crate::postgres::values_sql_lists;

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
}
