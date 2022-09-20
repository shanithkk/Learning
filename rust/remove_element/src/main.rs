fn main() {
    let mut res = vec![2,4,5,3,2,7,8];
    let f =remove_element(&mut res, 2);
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x!= val);
    nums.len() as i32
}