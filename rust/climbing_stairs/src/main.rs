
fn main() {
    let a = 2;
    let s = count_ways(a);
    println!("{}",s);
}

// fn fib(n: i32) -> i32
// {
//     if n <= 1{
//         return n;
//     }
//     return fib(n - 1) + fib(n - 2);
// }
fn count_ways(s: i32) -> i32
{
    // println!("{}",fib(s + 2)); 

    return fib(s + 1);
}

fn fib(n: i32) ->i32
{
    let mut a = 0; 
    let mut b = 1; 
    let mut c;
    let i:i32;
    if n == 0{
        return a;
    }
    for i in 2..=n
    {
        c = a + b;
       a = b;
       b = c;
    }
    return b;
}