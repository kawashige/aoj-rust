use std::io::Read;

pub fn print(nums: &Vec<(&str, i32, usize)>) {
    println!(
        "{}",
        nums.iter().map(|i| i.0).collect::<Vec<&str>>().join(" ")
    );
}

pub fn check_stable(nums: &Vec<(&str, i32, usize)>) -> String {
    let mut i = 0;
    while i < nums.len() {
        let mut j = i;
        while j < nums.len() - 1 && nums[i].1 == nums[j + 1].1 {
            j += 1;
            if nums[j - 1].2 > nums[j].2 {
                return "Not stable".to_string();
            }
        }
        i = j + 1;
    }
    "Stable".to_string()
}

pub fn bubble_sort(nums: &mut Vec<(&str, i32, usize)>) {
    for i in 0..nums.len() {
        for j in ((i + 1)..nums.len()).rev() {
            if nums[j].1 < nums[j - 1].1 {
                nums.swap(j, j - 1);
            }
        }
    }
    print(nums);
    println!("{}", check_stable(nums));
}

pub fn selection_sort(nums: &mut Vec<(&str, i32, usize)>) {
    for i in 0..nums.len() {
        let mut min = i;
        for j in i..nums.len() {
            if nums[j].1 < nums[min].1 {
                min = j;
            }
        }
        nums.swap(i, min);
    }
    print(nums);
    println!("{}", check_stable(nums));
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums = iter
        .enumerate()
        .map(|(i, n)| (n, n[1..].parse::<i32>().unwrap(), i))
        .collect::<Vec<(&str, i32, usize)>>();
    bubble_sort(&mut nums.clone());
    selection_sort(&mut nums);
}
