fn main() {
    let a = str_str("sadbutsad".to_owned(), "sad".to_owned());
    println!("{:?}", a);
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let idx = haystack.find(&needle);
    match idx {
        Some(x) => x as i32,
        None => -1,
    }
}
