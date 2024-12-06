use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    let mut left_numbers: Vec<i32> = vec![];

    let mut occurences: HashMap<i32, i32> = HashMap::new();

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

        let count = occurences.entry(r).or_insert(0);
        *count += 1;

        input.clear();
    }

    left_numbers.sort();

    let mut result: i32 = 0;

    for l in left_numbers.iter() {
        let count = occurences.get(l).unwrap_or(&0);

        result += l * count;
    }

    println!("{}", result);
}
