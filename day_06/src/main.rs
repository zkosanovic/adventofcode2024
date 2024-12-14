use indicatif::ProgressBar;
use std::io;

#[derive(Clone, PartialEq)]
enum Direction {
    No,
    Up,
    Right,
    Down,
    Left,
}

enum Error {
    LoopDetected,
    Unexpected,
}

fn walk_up(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    direction: &mut Vec<Vec<Direction>>,
) -> Result<(usize, usize), Error> {
    if direction[x][y] == Direction::Up {
        return Err(Error::LoopDetected);
    } else if direction[x][y] == Direction::No {
        direction[x][y] = Direction::Up;
    }

    let mut i = x;

    while i > 0 && v[i - 1][y] != '#' {
        i -= 1;
    }

    if i == 0 {
        return Ok((i, y));
    }

    return walk_right(i, y, v, direction);
}

fn walk_right(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    direction: &mut Vec<Vec<Direction>>,
) -> Result<(usize, usize), Error> {
    if direction[x][y] == Direction::Right {
        return Err(Error::LoopDetected);
    } else if direction[x][y] == Direction::No {
        direction[x][y] = Direction::Right;
    }

    let mut j = y;

    while j < v[x].len() - 1 && v[x][j + 1] != '#' {
        j += 1;
    }

    if j == v[x].len() - 1 {
        return Ok((x, j));
    }

    return walk_down(x, j, v, direction);
}

fn walk_down(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    direction: &mut Vec<Vec<Direction>>,
) -> Result<(usize, usize), Error> {
    if direction[x][y] == Direction::Down {
        return Err(Error::LoopDetected);
    } else if direction[x][y] == Direction::No {
        direction[x][y] = Direction::Down;
    }

    let mut i = x;

    while i < v.len() - 1 && v[i + 1][y] != '#' {
        i += 1;
    }

    if i == v.len() - 1 {
        return Ok((i, y));
    }

    return walk_left(i, y, v, direction);
}

fn walk_left(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    direction: &mut Vec<Vec<Direction>>,
) -> Result<(usize, usize), Error> {
    if direction[x][y] == Direction::Left {
        return Err(Error::LoopDetected);
    } else if direction[x][y] == Direction::No {
        direction[x][y] = Direction::Left;
    }

    let mut j = y;

    while j > 0 && v[x][j - 1] != '#' {
        j -= 1;
    }

    if j == 0 {
        return Ok((x, j));
    }

    return walk_up(x, j, v, direction);
}

fn walk(v: &Vec<Vec<char>>) -> Result<(usize, usize), Error> {
    let mut direction = vec![vec![Direction::No; v[0].len()]; v.len()];

    for (x, row) in v.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            match ch {
                '^' => {
                    return walk_up(x, y, &v, &mut direction);
                }
                '>' => {
                    return walk_right(x, y, &v, &mut direction);
                }
                'v' => {
                    return walk_down(x, y, &v, &mut direction);
                }
                '<' => {
                    return walk_left(x, y, &v, &mut direction);
                }
                _ => continue,
            }
        }
    }
    Err(Error::Unexpected)
}

fn main() {
    let mut v: Vec<Vec<char>> = Vec::new();

    let mut input = String::new();

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }

        v.push(input.trim().chars().collect());

        input.clear();
    }

    let bar = ProgressBar::new((v.len() * v[0].len()).try_into().unwrap());

    let mut result = 0;

    for x in 0..v.len() {
        for y in 0..v[x].len() {
            bar.inc(1);

            if v[x][y] != '.' {
                continue;
            }

            v[x][y] = '#';

            match walk(&v) {
                Err(Error::LoopDetected) => {
                    result += 1;
                }
                _ => (),
            }

            v[x][y] = '.';
        }
    }

    bar.finish_and_clear();

    println!("Result: {}", result);
}
