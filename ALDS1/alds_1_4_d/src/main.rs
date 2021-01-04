use std::io::Read;

pub fn check(nums: &[i32], p: i32, k: i32) -> usize {
    let mut i = 0;
    for _ in 0..k {
        let mut w = 0;
        while w + nums[i] <= p {
            w += nums[i];
            i += 1;
            if i == nums.len() {
                return i;
            }
        }
    }
    i
}

pub fn solve(nums: Vec<i32>, tracks: i32) {
    let mut left = 0;
    let mut right = 100000 * 10000;
    while 1 < right - left {
        let mid = (left + right) / 2;
        let n = check(&nums, mid, tracks);
        if nums.len() <= n {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let mut first_line = iter.next().unwrap().split(' ');
    let _ = first_line.next();
    let tracks = first_line.next().unwrap().parse().unwrap();
    let nums = iter
        .filter(|s| !s.is_empty())
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    solve(nums, tracks);
}
