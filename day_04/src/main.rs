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
            if *cell != 'A' || i < 1 || j < 1 || i >= n - 1 || j >= m - 1 {
                continue;
            }

            if ((mat[i - 1][j - 1] == 'M' && mat[i + 1][j + 1] == 'S')
                || (mat[i - 1][j - 1] == 'S' && mat[i + 1][j + 1] == 'M'))
                && ((mat[i - 1][j + 1] == 'M' && mat[i + 1][j - 1] == 'S')
                    || (mat[i - 1][j + 1] == 'S' && mat[i + 1][j - 1] == 'M'))
            {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
