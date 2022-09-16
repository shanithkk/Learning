/// Finding Common prefix in an list of an array

fn main() {
    let words = ["aaa", "aa", "aaa"];

    let first = words[0];
    let first_lenght = words[0].len();
    let mut vec: Vec<char> = Vec::new();
    let mut flag = false;
    let mut res = "".to_string();

    if words.len() == 1 {
        res = first.to_string();
    } else {
        for i in 0..first_lenght {
            for j in 1..words.len() {
                if first.chars().nth(i) == words[j].chars().nth(i) {
                    flag = true;
                } else {
                    flag = false;
                    break;
                }
            }
            if flag {
                vec.push(first.chars().nth(i).unwrap());
            }
            if flag == false {
                break;
            }
        }
        if vec.is_empty() {
            res = "".to_string();
        } else {
            res = vec.iter().collect();
        }
    }

    println!("{:?}", res);
}
