use std::collections::HashSet;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(matrix: [[usize; 3]; 3]) {
    let goal = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];
    let mut count = 0;
    let mut seen = HashSet::new();
    let mut next = vec![matrix];

    while !next.is_empty() {
        let mut new_next = Vec::new();
        for n in next {
            if n == goal {
                println!("{}", count);
                return;
            }
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n);
            for i in 0..3 {
                for j in 0..3 {
                    if n[i][j] == 0 {
                        if 0 < i {
                            let mut next = n.clone();
                            next[i][j] = next[i - 1][j];
                            next[i - 1][j] = 0;
                            new_next.push(next);
                        }
                        if i < 2 {
                            let mut next = n.clone();
                            next[i][j] = next[i + 1][j];
                            next[i + 1][j] = 0;
                            new_next.push(next);
                        }
                        if 0 < j {
                            let mut next = n.clone();
                            next[i][j] = next[i][j - 1];
                            next[i][j - 1] = 0;
                            new_next.push(next);
                        }
                        if j < 2 {
                            let mut next = n.clone();
                            next[i][j] = next[i][j + 1];
                            next[i][j + 1] = 0;
                            new_next.push(next);
                        }
                    }
                }
            }
        }
        count += 1;
        next = new_next;
    }
}

fn main() {
    let mut matrix = [[0; 3]; 3];
    for i in 0..3 {
        let v = read::<String>()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        matrix[i][0] = v[0];
        matrix[i][1] = v[1];
        matrix[i][2] = v[2];
    }
    solve(matrix);
}
