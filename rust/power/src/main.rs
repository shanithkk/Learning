fn main() {
    let n: i32 =-2;let x =2_f64;

    // let len= n.abs();

    let mut actual_result = 0_f64;
        
        if n<0{
            
            let mut result = 1_f64;
            for i in 0..n.abs(){
                result = result * x;
            }
            println!("{result}");
            actual_result = 1_f64/ result;
            
        }
        else if n>0{
            let mut result = 1_f64;
            for i in 0..n{
                result = result *x;
            }
            actual_result = result;
        }
        else{
            actual_result = 1.0;

        }
        
        println!("{}", actual_result);
}
