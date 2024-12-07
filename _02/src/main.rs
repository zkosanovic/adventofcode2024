use std::io;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
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

        let mut row = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        let first = match row.next() {
            Some(x) => x,
            None => continue,
        };

        let second = match row.next() {
            Some(x) => x,
            None => continue,
        };

        if first == second || (first - second).abs() > 3 {
            continue;
        }

        let direction = if first < second {
            Direction::Increasing
        } else {
            Direction::Decreasing
        };

        let mut previous = second;
        let mut ok = true;

        for x in row {
            if (direction == Direction::Increasing && previous >= x) || (direction == Direction::Decreasing && previous <= x) || ((x - previous).abs() > 3) {
                ok = false;
                break;
            }

            previous = x;
        }

        if !ok {
            continue;
        }

        result += 1;
    }

    println!("{}", result);
}
