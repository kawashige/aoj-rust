use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn recurse(m: u128, n: u128) -> u128 {
    if n == 1 {
        return m;
    }
    let result = recurse(m, n / 2);
    if n % 2 == 0 {
        (result * result) % 100_000_0007
    } else {
        (result * result * m) % 100_000_0007
    }
}

fn solve(m: u128, n: u128) {
    println!("{}", recurse(m, n));
}

fn main() {
    let (a, b) = {
        let v = read::<String>()
            .split_whitespace()
            .map(|n| n.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        (v[0], v[1])
    };
    solve(a, b);
}
