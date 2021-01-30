use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(n: usize, coins: Vec<usize>) {
    let mut dp = [false; 50001];
    dp[0] = true;
    for j in 0..n {
        let mut tmp = [false; 50001];
        tmp[0] = true;
        for i in 0..dp.len() {
            if !dp[i] {
                continue;
            }
            for c in &coins {
                if i + c < dp.len() {
                    tmp[i + c] = true;
                }
            }
        }
        dp = tmp;
        if dp[n] {
            println!("{}", j + 1);
            return;
        }
    }
}

fn main() {
    let n = {
        let v = read::<String>()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        v[0]
    };

    let coins = read::<String>()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    solve(n, coins);
}
