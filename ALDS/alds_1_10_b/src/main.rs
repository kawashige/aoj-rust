use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(matrixes: Vec<(usize, usize)>) {
    let mut dp = vec![vec![0; matrixes.len()]; matrixes.len()];

    for j in 1..matrixes.len() {
        for i in 0..(matrixes.len() - j) {
            let mut min = std::usize::MAX;
            for k in (i + 1)..=(i + j) {
                min = std::cmp::min(
                    min,
                    dp[i][k - 1] + dp[k][i + j] + matrixes[i].0 * matrixes[k].0 * matrixes[i + j].1,
                );
            }
            dp[i][i + j] = min;
        }
    }

    println!("{}", dp[0][dp.len() - 1]);
}

fn main() {
    let n: usize = read();
    let matrixes: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let s: String = read();
            let v = s
                .split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (v[0], v[1])
        })
        .collect();
    solve(matrixes);
}
