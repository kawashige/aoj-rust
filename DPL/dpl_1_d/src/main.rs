use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(nums: Vec<usize>) {
    let mut lens = vec![0; nums.len()];
    lens[0] = nums[0];
    let mut j = 1;

    for i in 1..nums.len() {
        if lens[j - 1] < nums[i] {
            lens[j] = nums[i];
            j += 1;
        } else {
            let k = match lens[..j].binary_search(&nums[i]) {
                Ok(n) => n,
                Err(n) => n,
            };
            lens[k] = nums[i];
        }
    }

    println!("{}", j);
}

fn main() {
    let n: usize = read();
    let nums = (0..n).map(|_| read::<usize>()).collect();
    solve(nums);
}
