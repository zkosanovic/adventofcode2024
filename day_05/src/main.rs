use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn main() {
    let mut before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut after: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut location: HashMap<usize, usize> = HashMap::new();

    let mut input = String::new();

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }

        if input.trim() == "" {
            break;
        }

        let order_input: Vec<usize> = input
            .trim()
            .split('|')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if order_input.len() != 2 {
            panic!("Invalid input");
        }

        let (l, r) = (order_input[0], order_input[1]);

        if !before.contains_key(&r) {
            before.insert(r, HashSet::new());
        }

        before.get_mut(&r).unwrap().insert(l);

        if !after.contains_key(&l) {
            after.insert(l, HashSet::new());
        }

        after.get_mut(&l).unwrap().insert(r);

        input.clear();
    }

    let mut result: usize = 0;

    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            // EOF
            break;
        }

        let pages: Vec<usize> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        for (i, &page) in pages.iter().enumerate() {
            location.insert(page, i);
        }

        let mut ok = true;
        for (i, page) in pages.iter().enumerate() {
            if let Some(a) = after.get(page) {
                for j in a.iter() {
                    if let Some(j) = location.get(j) {
                        if i > *j {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if !ok {
                break;
            }

            if let Some(b) = before.get(page) {
                for j in b.iter() {
                    if let Some(j) = location.get(j) {
                        if i < *j {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if !ok {
                break;
            }
        }

        if ok {
            result += pages[pages.len() / 2];
        }

        location.clear();
        input.clear();
    }

    println!("{}", result);
}
