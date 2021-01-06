use std::io::Read;

pub fn partition(nums: &mut Vec<i32>, p: usize, r: usize) -> usize {
    let x = nums[r];
    let mut i = p;
    for j in p..r {
        if nums[j] <= x {
            i += 1;
            nums.swap(i - 1, j)
        }
    }
    nums.swap(i, r);
    i
}

pub fn solve(mut nums: Vec<i32>) {
    let l = nums.len();
    let count = partition(&mut nums, 0, l - 1);
    println!(
        "{}",
        nums.iter()
            .enumerate()
            .map(|(i, n)| if i == count {
                format!("[{}]", n)
            } else {
                n.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let nums = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    solve(nums);
}
