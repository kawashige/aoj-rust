use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(nums: Vec<Vec<usize>>) {
    let mut dp = vec![vec![0; nums[0].len()]; nums.len()];

    let mut max = 0;
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            if nums[i][j] == 1 {
                dp[i][j] = 0;
            } else {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] =
                        std::cmp::min(std::cmp::min(dp[i - 1][j], dp[i - 1][j - 1]), dp[i][j - 1])
                            + 1;
                }
            }
            max = std::cmp::max(max, dp[i][j]);
        }
    }
    println!("{}", max * max);
}

fn main() {
    let n: usize = {
        let nums = read::<String>()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        nums[0]
    };
    let nums = (0..n)
        .map(|_| {
            read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(nums);
}
