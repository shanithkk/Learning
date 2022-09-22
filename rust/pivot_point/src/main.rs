fn main() {
    let a  = vec![1,7,3,6,5,6];
    let s =pivot_point(a);
    println!("{}",s);
}

fn pivot_point(s: Vec<i32>)->i32{
    let mut sum :i32 = s.iter().sum();
    
    let mut t = 0;

    for i in 0..s.len(){
        sum -= s[i];
        if sum == t {return  i as i32;}
        t += s[i];
    }
    
    -1 
}