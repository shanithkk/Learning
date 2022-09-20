use std::num::Wrapping;

fn main() {
    println!("Hello, world!");
    let res = divide(-2147483648, -1);
    println!("{res}");
}

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    // match  Wrapping(dividend)/Wrapping(divisor) {
    //     x => x.0,
    //     _ => i32::MAX,
    // } 
    if dividend == -2147483648 && divisor == -1{
        return 2147483647;
    }
    //a = dividend/divisor;
    let a =Wrapping(dividend)/Wrapping(divisor);
    a.0
}