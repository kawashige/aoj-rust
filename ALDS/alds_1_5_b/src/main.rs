use std::io::Read;

pub fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
    let mut l = Vec::new();
    for i in 0..(mid - left) {
        l.push(nums[left + i]);
    }
    let mut r = Vec::new();
    for i in 0..(right - mid) {
        r.push(nums[mid + i]);
    }
    l.push(std::i32::MAX);
    r.push(std::i32::MAX);
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    for k in left..right {
        count += 1;
        if l[i] <= r[j] {
            nums[k] = l[i];
            i += 1;
        } else {
            nums[k] = r[j];
            j += 1;
        }
    }
    count
}

pub fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
    let mut count = 0;
    if left + 1 < right {
        let mid = (left + right) / 2;
        count += merge_sort(nums, left, mid);
        count += merge_sort(nums, mid, right);
        count += merge(nums, left, mid, right);
    }
    count
}

pub fn solve(mut nums: Vec<i32>) {
    let l = nums.len();
    let count = merge_sort(&mut nums, 0, l);
    println!(
        "{}",
        nums.into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("{}", count);
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
