use std::collections::{HashMap, HashSet};
use std::io::Read;

pub fn minimum_cost_sort(nums: Vec<i32>) -> i32 {
    let mut sorted = nums.clone();
    sorted.sort();
    let min = sorted[0];
    let map = sorted.into_iter().zip(0..).collect::<HashMap<i32, usize>>();

    let mut cost = 0;
    let mut opened = HashSet::new();
    for i in 0..nums.len() {
        if opened.contains(&i) {
            continue;
        }
        let mut next = map.get(&nums[i]).unwrap();
        let mut tmp_min = nums[i];
        let mut values = vec![nums[i]];
        opened.insert(next);
        while next != &i {
            tmp_min = std::cmp::min(tmp_min, nums[*next]);
            values.push(nums[*next]);
            next = map.get(&nums[*next]).unwrap();
            opened.insert(next);
        }
        if 1 < values.len() {
            cost += std::cmp::min(
                values.iter().sum::<i32>() + (values.len() as i32 - 2) * tmp_min,
                values.iter().sum::<i32>() + tmp_min + (values.len() as i32 + 1) * min,
            );
        }
    }
    cost
}

pub fn solve(nums: Vec<i32>) {
    let cost = minimum_cost_sort(nums);
    println!("{}", cost);
}

fn main() {
    let nums = {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let mut iter = buf.split('\n');
        let _n = iter.next().unwrap();
        iter.next()
            .unwrap()
            .split(' ')
            .map(|i| i.parse::<i32>().unwrap())
            .collect()
    };
    solve(nums);
}
