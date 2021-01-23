use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(strings: Vec<(String, String)>) {
    for (s1, s2) in strings {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let mut dp = vec![vec![0; b2.len() + 1]; b1.len() + 1];
        for i in 0..b1.len() {
            for j in 0..b2.len() {
                dp[i + 1][j + 1] = if b1[i] == b2[j] {
                    1 + dp[i][j]
                } else {
                    std::cmp::max(dp[i][j + 1], dp[i + 1][j])
                };
            }
        }
        println!("{}", dp[b1.len()][b2.len()]);
    }
}

fn main() {
    let n: usize = read();
    let strings: Vec<(String, String)> = (0..n).map(|_| (read(), read())).collect();
    solve(strings);
}
