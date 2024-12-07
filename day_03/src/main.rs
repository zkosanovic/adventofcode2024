use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do)(\(\))|(don)'t(\(\))").unwrap();

    let mut input = String::new();

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }
    }

    let mut result: u64 = 0;
    let mut on = true;

    for cap in re.captures_iter(&input) {
        let (s, nums): (&str, [&str; 2]) = cap.extract();

        if s == "don't()" {
            on = false;
            continue;
        }

        if s == "do()" {
            on = true;
            continue;
        }

        if !on {
            continue;
        }

        let x = nums[0].parse::<u64>().unwrap();
        let y = nums[1].parse::<u64>().unwrap();

        result += x * y;
    }

    println!("{}", result);
}
