use std::io;

fn main() {
    let mut input = String::new();

    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }

        let mut splitted = input.split_whitespace();

        let l = match splitted.next() {
            Some(s) => s.parse::<i32>().unwrap(),
            None => panic!("Bad input"),
        };

        left_numbers.push(l);

        let r = match splitted.next() {
            Some(s) => s.parse::<i32>().unwrap(),
            None => panic!("Bad input"),
        };

        right_numbers.push(r);

        input.clear();
    }

    left_numbers.sort();
    right_numbers.sort();

    if left_numbers.len() != right_numbers.len() {
        panic!("Left and right numbers not the same length");
    }

    let mut result: i32 = 0;

    for (l, r) in left_numbers.iter().zip(right_numbers.iter()) {
        result += (l - r).abs();
    }

    println!("{}", result);
}
