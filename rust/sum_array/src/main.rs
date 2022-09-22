fn main() {
    let nums = [1, 2, 3, 4, 5];
    let target = 10;
    println!("{:?}", sum(nums.to_vec(), target));
}

fn sum(a: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[i] + a[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
