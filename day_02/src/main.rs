use std::io;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(row: &[i32]) -> bool {
    let first = row[0];
    let second = row[1];

    if first == second || (first - second).abs() > 3 {
        return false;
    }

    let direction = if first < second {
        Direction::Increasing
    } else {
        Direction::Decreasing
    };

    let mut previous = second;

    for x in row.iter().skip(2) {
        if (direction == Direction::Increasing && previous >= *x)
            || (direction == Direction::Decreasing && previous <= *x)
            || ((x - previous).abs() > 3)
        {
            return false;
        }

        previous = *x;
    }

    true
}

fn is_safe_without(row: &[i32], without: usize) -> bool {
    let first: i32;
    let second: i32;
    let start: usize;

    match without {
        0 => {
            first = row[1];
            second = row[2];
            start = 3;
        }
        1 => {
            first = row[0];
            second = row[2];
            start = 3;
        }
        _ => {
            first = row[0];
            second = row[1];
            start = 2;
        }
    }

    if first == second || (first - second).abs() > 3 {
        return false;
    }

    let direction = if first < second {
        Direction::Increasing
    } else {
        Direction::Decreasing
    };

    let mut previous = second;

    for (i, x) in row.iter().enumerate().skip(start) {
        if i == without {
            continue;
        }

        if (direction == Direction::Increasing && previous >= *x)
            || (direction == Direction::Decreasing && previous <= *x)
            || ((x - previous).abs() > 3)
        {
            return false;
        }

        previous = *x;
    }

    true
}

fn main() {
    let mut input = String::new();

    let mut result: u32 = 0;

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }

        let line = input.clone();
        input.clear();

        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&row) {
            result += 1;
            continue;
        }

        for i in 0..row.len() {
            if is_safe_without(&row, i) {
                result += 1;
                break;
            }
        }
    }

    println!("{}", result);
}
