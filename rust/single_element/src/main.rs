fn main() {
    let nums = [1, 1, 2].to_vec();
    println!("{}", single_element(nums));
}

fn single_element(nums: Vec<i32>) -> i32 {
    let mut ans = -1;
    let mut i = 0;

    if nums.len() == 1 {
        return nums[0];
    }

    let n = nums.len();
    while i < nums.len() {
        if i + 1 >= nums.len() {
            break;
        }
        if nums[i] != nums[i + 1] {
            return nums[i];
        };
        i = i + 2;
    }
    if nums[n - 2] != nums[n - 1] {
        ans = nums[n - 1];
    }

    ans
}
