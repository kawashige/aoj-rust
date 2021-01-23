use std::io::Read;

pub fn find(nums: &[i32], target: i32) -> bool {
    for i in 0..nums.len() {
        if nums[i] == target {
            return true;
        } else if nums[i] < target {
            if find(&nums[(i + 1)..], target - nums[i]) {
                return true;
            }
        }
    }
    false
}

pub fn solve(nums1: Vec<i32>, nums2: Vec<i32>) {
    for n in nums2 {
        if find(&nums1, n) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n1 = iter.next().unwrap();
    let nums1 = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    let _n2 = iter.next().unwrap();
    let nums2 = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    solve(nums1, nums2);
}
