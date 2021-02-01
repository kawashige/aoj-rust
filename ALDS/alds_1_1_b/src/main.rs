use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn solve(a: usize, b: usize) {
    println!("{}", gcd(a, b));
}

fn main() {
    let (a, b) = {
        let v = read::<String>()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (v[0], v[1])
    };
    solve(a, b);
}
