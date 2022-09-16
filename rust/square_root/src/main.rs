fn main() {
    let x =2147395600;

    println!("{:?}", sqrt_root(x));
}


fn sqrt_root(x: i32) ->i32
{
    if x == 0 || x == 1{
        return x;
    }
    let mut i = 1; let mut result = 1_u64;
    while result <= x as u64 {
        i =i+1;
        result = i * i;
    }
    return (i - 1).try_into().unwrap();
}