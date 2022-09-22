fn main() {
    let num = vec![1, 2, 3, 4];
    println!("{:?}", sum_vector(num));
}

fn sum_vector(s: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut sum = 0;

    for i in 0..s.len() {
        sum += s[i];
        res.push(sum);
    }
    res
}
