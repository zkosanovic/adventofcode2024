use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut input = String::new();

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }
    }

    let mut result: u64 = 0;

    for cap in re.captures_iter(&input) {
        let (_, nums): (&str, [&str; 2]) = cap.extract();

        let x = nums[0].parse::<u64>().unwrap();
        let y = nums[1].parse::<u64>().unwrap();

        result += x * y;
    }

    println!("{}", result);
}
