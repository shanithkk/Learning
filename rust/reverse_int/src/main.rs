fn main() {
    let a = 1534236469;
    let str = a.to_string();
    let mut result: String = String::new();

    let mut tr = "";
    if a < 0 {
        tr = str.trim_start_matches("-");
        result = tr.to_string().chars().rev().collect();
        result.insert(0, '-');
    } else if a > 0 {
        result = a.to_string().chars().rev().collect();
    } else {
        result = "0".to_string();
    }
    let s = match result.parse::<i32>() {
        Ok(s) => s,
        Err(e) => 0,
    };
    println!("{s}");
}
