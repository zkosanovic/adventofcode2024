use std::io;

fn walk_up(x: usize, y: usize, v: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> (usize, usize) {
    visited[x][y] = true;

    let mut i = x;

    while i > 0 && v[i - 1][y] != '#' {
        i -= 1;
        visited[i][y] = true;
    }

    if i == 0 {
        return (i, y);
    }

    return walk_right(i, y, v, visited);
}

fn walk_right(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[x][y] = true;

    let mut j = y;

    while j < v[x].len() - 1 && v[x][j + 1] != '#' {
        j += 1;
        visited[x][j] = true;
    }

    if j == v[x].len() - 1 {
        return (x, j);
    }

    return walk_down(x, j, v, visited);
}

fn walk_down(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[x][y] = true;

    let mut i = x;

    while i < v.len() - 1 && v[i + 1][y] != '#' {
        i += 1;
        visited[i][y] = true;
    }

    if i == v.len() - 1 {
        return (i, y);
    }

    return walk_left(i, y, v, visited);
}

fn walk_left(
    x: usize,
    y: usize,
    v: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[x][y] = true;

    let mut j = y;

    while j > 0 && v[x][j - 1] != '#' {
        j -= 1;
        visited[x][j] = true;
    }

    if j == 0 {
        return (x, j);
    }

    return walk_up(x, j, v, visited);
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

    let mut visited = vec![vec![false; v[0].len()]; v.len()];

    for (x, row) in v.iter().enumerate() {
        for (y, ch) in row.iter().enumerate() {
            match ch {
                '^' => {
                    walk_up(x, y, &v, &mut visited);
                }
                '>' => {
                    walk_right(x, y, &v, &mut visited);
                }
                'v' => {
                    walk_down(x, y, &v, &mut visited);
                }
                '<' => {
                    walk_left(x, y, &v, &mut visited);
                }
                _ => continue,
            }
        }
    }

    let mut result = 0;

    for row in visited.iter() {
        for v in row.iter() {
            if *v {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}
