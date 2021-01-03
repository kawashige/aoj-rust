use std::io::Read;

pub fn exec(mut nums1: Vec<i32>, mut nums2: Vec<i32>) {
    nums1.sort();
    nums2.sort();
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            count += 1;
            i += 1;
            j += 1;
        } else if nums1[i] < nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
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
