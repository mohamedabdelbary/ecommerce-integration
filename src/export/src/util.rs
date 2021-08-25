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