use std::io::Read;

pub fn print(nums: &Vec<i32>) {
    println!(
        "{}",
        nums.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn bubble_sort(nums: &mut Vec<i32>) {
    let mut count = 0;
    let mut flag = true;
    while flag {
        flag = false;
        for i in (1..nums.len()).rev() {
            if nums[i] < nums[i - 1] {
                nums.swap(i, i - 1);
                count += 1;
                flag = true;
            }
        }
    }
    print(nums);
    println!("{}", count);
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums = iter.map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>();
    bubble_sort(&mut nums);
}
