use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(nums: Vec<usize>) {
    let max = *nums.iter().max().unwrap();
    let max_sqrt = (max as f32).sqrt() as usize;
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=max {
        if !is_prime[i] {
            continue;
        }
        if i <= max_sqrt {
            for j in (2..).take_while(|j| j * i <= max) {
                is_prime[i * j] = false;
            }
        }
    }
    println!("{}", nums.iter().filter(|n| is_prime[**n]).count());
}

fn main() {
    let n: usize = read();
    let nums = (0..n).map(|_| read::<usize>()).collect();
    solve(nums);
}
