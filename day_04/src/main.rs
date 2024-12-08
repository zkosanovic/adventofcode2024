use std::io;

fn main() {
    let mut input = String::new();

    let mut mat: Vec<Vec<char>> = vec![];

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            //EOF
            break;
        }

        mat.push(input.trim().chars().collect());

        input.clear();
    }

    let n = mat.len();
    let m = mat[0].len();

    let mut result = 0;

    for (i, row) in mat.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != 'X' {
                continue;
            }

            if i < n - 3 && mat[i+1][j] == 'M' && mat[i+2][j] == 'A' && mat[i+3][j] == 'S' {
                result += 1;
            }

            if i > 2 && mat[i-1][j] == 'M' && mat[i-2][j] == 'A' && mat[i-3][j] == 'S' {
                result += 1;
            }

            if j < m - 3 && mat[i][j+1] == 'M' && mat[i][j+2] == 'A' && mat[i][j+3] == 'S' {
                result += 1;
            }

            if j > 2 && mat[i][j-1] == 'M' && mat[i][j-2] == 'A' && mat[i][j-3] == 'S' {
                result += 1;
            }

            if i > 2 && j > 2 && mat[i-1][j-1] == 'M' && mat[i-2][j-2] == 'A' && mat[i-3][j-3] == 'S' {
                result += 1;
            }

            if i > 2 && j < m - 3 && mat[i-1][j+1] == 'M' && mat[i-2][j+2] == 'A' && mat[i-3][j+3] == 'S' {
                result += 1;
            }

            if i < n - 3 && j > 2 && mat[i+1][j-1] == 'M' && mat[i+2][j-2] == 'A' && mat[i+3][j-3] == 'S' {
                result += 1;
            }

            if i < n - 3 && j < m - 3 && mat[i+1][j+1] == 'M' && mat[i+2][j+2] == 'A' && mat[i+3][j+3] == 'S' {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
