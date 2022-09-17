fn main() {
    let x = 121;
    println!("{}", is_palindrome(x));
}

pub fn is_palindrome(x: i32) -> bool {
        let num = x.to_string();
        let rev :String= num.chars().rev().collect();
        rev==num
}