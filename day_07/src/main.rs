use std::io;

fn backtrack(current: u64, goal: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 0 {
        return current == goal;
    }

    if current > goal {
        return false;
    }

    let mut new_current = current.to_string();
    new_current.push_str(&numbers[0].to_string());

    return backtrack(current * numbers[0], goal, &numbers[1..])
        || backtrack(current + numbers[0], goal, &numbers[1..])
        || backtrack(new_current.parse::<u64>().unwrap(), goal, &numbers[1..]);
}

fn main() {
    let mut input = String::new();

    let mut result: u64 = 0;

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            break;
        }

        let i = input.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
        if i.len() != 2 {
            panic!("Invalid input");
        }

        let goal = i[0].parse::<u64>().unwrap();
        let numbers = i[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        if numbers.len() == 0 {
            panic!("Invalid input");
        }

        if backtrack(numbers[0], goal, &numbers[1..]) {
            result += goal;
        }

        input.clear();
    }

    println!("{}", result);
}
