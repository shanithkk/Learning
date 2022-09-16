fn main() {
    let roman = "MCMXCIV";
    let vect: Vec<char> = roman.chars().collect();
    let mut sum = 0;
    let lenght = vect.len();

    for (idx, v) in vect.iter().enumerate() {
        if idx < (lenght - 1) && roman_value(&vect[idx]) < roman_value(&vect[idx + 1]) {
            sum = sum - roman_value(v);
        } else {
            sum = sum + roman_value(v);
        }
    }
    println!("{}", sum);
}

fn roman_value(letter: &char) -> i32 {
    match letter {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
