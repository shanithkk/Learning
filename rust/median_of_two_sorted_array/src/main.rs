fn main() {
    println!("Hello, world!");
    let a = vec![1, 3];
    let b = vec![2,4, 7];
    let c = Solution::find_median_sorted_arrays(a, b);
    println!("{c:?}");
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut combined: Vec<i32> = nums1;
        combined.extend(nums2);
        combined.sort();
        let sum:f64;

        let len = combined.len();
        if combined.len() %2 ==0{
            sum = (combined[(len/2)-1]+ combined[len/2]) as f64/ 2 as f64;
        }else {
            sum = combined[len/2] as f64
        }

        sum
    }
}
