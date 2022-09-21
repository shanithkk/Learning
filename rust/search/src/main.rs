fn main() {
    println!("Hello, world!");
    let nums = vec![3,4,5,6,0,1,2];
    println!("{}", search(nums, 4))
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    
    match nums.iter().position(|&x| x==target){
        Some(x) => x as i32,
        None => -1,
    }
}