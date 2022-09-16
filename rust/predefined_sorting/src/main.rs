fn main() {
    let pre_order = ["SM", "MD", "LG", "XL", "2X","3X","4X", "5X"];
    // let input = ["3X","LG","MD","XL","4X","SM"];
    let input = ["LG","SM","4X","SM","3X" ];
    let mut res: Vec<&str> = Vec::new();

    for i in pre_order.iter(){
        for wrd in input{
            if *i == wrd{
                res.push(wrd);
            }
        }
    }

    println!("{:?}", res);
}
