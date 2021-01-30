use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(w: usize, items: Vec<Vec<usize>>) {
    let mut dp = vec![vec![0; w + 1]; items.len() + 1];

    for i in 0..items.len() {
        for j in 0..=w {
            if items[i][1] <= j {
                dp[i + 1][j] = std::cmp::max(dp[i][j], dp[i][j - items[i][1]] + items[i][0]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[items.len()][w]);
}

fn main() {
    let (n, w) = {
        let v = read::<String>()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (v[0], v[1])
    };

    let items = (0..n)
        .map(|_| {
            read::<String>()
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(w, items);
}
