use std::io::Read;

pub fn binary_search(nums: &Vec<i32>, target: i32) -> bool {
    let mut s = 0;
    let mut e = nums.len() - 1;
    while s <= e {
        let mid = s + (e - s) / 2;
        if nums[mid] == target {
            return true;
        } else if target < nums[mid] {
            e = mid - 1;
        } else {
            s = mid + 1;
        }
    }
    false
}

pub fn exec(nums1: Vec<i32>, nums2: Vec<i32>) {
    let count = nums2
        .into_iter()
        .filter(|n| binary_search(&nums1, *n))
        .count();
    println!("{}", count);
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _ = iter.next().unwrap().parse::<usize>().unwrap();
    let nums1 = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    let _ = iter.next().unwrap().parse::<usize>().unwrap();
    let nums2 = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    exec(nums1, nums2);
}
