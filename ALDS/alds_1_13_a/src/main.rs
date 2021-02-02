use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn backtracking(queens: &mut Vec<Vec<usize>>) -> bool {
    if queens.len() == 8 {
        return true;
    }
    for i in 0..8 {
        for j in 0..8 {
            if queens.iter().any(|q| {
                i == q[0]
                    || j == q[1]
                    || (i as i32 - q[0] as i32).abs() == (j as i32 - q[1] as i32).abs()
            }) {
                continue;
            }
            queens.push(vec![i, j]);
            if backtracking(queens) {
                return true;
            }
            queens.pop();
        }
    }
    false
}

fn solve(mut queens: Vec<Vec<usize>>) {
    backtracking(&mut queens);
    let mut result = vec![vec!['.'; 8]; 8];
    for q in queens {
        result[q[0]][q[1]] = 'Q';
    }
    for row in result {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    let n: usize = read();
    let queens = (0..n)
        .map(|_| {
            read::<String>()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(queens);
}
