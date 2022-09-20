use std::num::Wrapping;

fn main() {
    println!("Hello, world!");
    let res = divide(-2147483648, -1);
    println!("{res}");
}

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == -2147483648 && divisor == -1{
        return 2147483647;
    }
    let a =Wrapping(dividend)/Wrapping(divisor);
    a.0
}