use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(nums: Vec<i32>) {
    for i in 0..nums.len() {
        let mut s = vec![format!("node {}: key = {},", i + 1, nums[i])];
        if 0 < i {
            s.push(format!("parent key = {},", nums[(i + 1) / 2 - 1]));
        }
        if (i + 1) * 2 - 1 < nums.len() {
            s.push(format!("left key = {},", nums[(i + 1) * 2 - 1]));
        }
        if (i + 1) * 2 < nums.len() {
            s.push(format!("right key = {},", nums[(i + 1) * 2]));
        }
        println!("{}", s.join(" "));
    }
}

fn main() {
    let _: usize = read();
    let nums: Vec<i32> = read::<String>()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    solve(nums);
}
