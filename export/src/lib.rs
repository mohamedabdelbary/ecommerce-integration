pub mod postgres;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::util::{chunks, wrap_with, dt_to_string};
    use crate::postgres::values_sql_lists;
    use chrono::prelude::*;
    use chrono::Utc;

    #[test]
    fn test_chunks() {
        let l = vec![1, 2, 3, 4];
        assert_eq!(chunks(&l, 1), vec![vec![1] , vec![2], vec![3], vec![4]]);
        assert_eq!(chunks(&l, 2), vec![vec![1 , 2], vec![3, 4]]);
        assert_eq!(chunks(&l, 3), vec![vec![1 , 2, 3], vec![4]]);
        assert_eq!(chunks(&l, 4), vec![vec![1 , 2, 3, 4]]);
    }

    #[test]
    fn test_wrap_with() {
        let s = "some string";
        let wrap = "|";
        assert_eq!(wrap_with(s, wrap), String::from("|some string|"));
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

    #[test]
    fn test_dt_to_string() {
        let dt = Utc.ymd(2021, 7, 8).and_hms(8, 10, 11); // `2021-07-08T08:10:11Z`
        let dt_string = dt_to_string(&dt);
        assert_eq!(dt_string, String::from("2021-07-08 08:10:11+00"));
    }
}