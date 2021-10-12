use chrono::{DateTime, FixedOffset, Datelike, Timelike};

pub fn chunks<T: Clone>(items: &Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::<Vec<T>>::new();
    let mut chunk = Vec::<T>::new();
    let mut i = 1;
    for item in items.iter() {
        chunk.push(item.clone());
        if i % chunk_size == 0 {
            result.push(chunk.clone());
            chunk.clear();
        }
        i += 1;
    }
    if chunk.len() > 0 {result.push(chunk);}
    result
}

pub fn wrap_with(s: &str, with: &str) -> String {
    let with_clone = with.clone();
    String::from(with_clone) + s.clone() + with_clone
}

fn date_string(dt: &DateTime<chrono::offset::Utc>) -> String {
    String::from(vec![
        dt.date().year().to_string(),
        format!("{:0width$}", dt.date().month(), width = 2),
        format!("{:0width$}", dt.date().day(), width = 2),
    ].join("-"))
}

fn hour_min_sec_string(dt: &DateTime<chrono::offset::Utc>) -> String {
    String::from(
        vec![
            format!("{:0width$}", dt.hour(), width = 2),
            format!("{:0width$}", dt.minute(), width = 2),
            format!("{:0width$}", dt.second(), width = 2)
        ].join(":")
    )
}

pub fn dt_to_string(dt: &DateTime<chrono::offset::Utc>) -> String {
    // We know it's UTC so hardcoding +00
    // TODO: Can be fetched from DB
    String::from(vec![date_string(&dt), hour_min_sec_string(&dt)].join(" ")) + "+00"
}
