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

pub fn selection_sort(nums: &mut Vec<i32>) {
    let mut count = 0;
    for i in 0..nums.len() {
        let mut min = i;
        for j in (i + 1)..nums.len() {
            if nums[j] < nums[min] {
                min = j;
            }
        }
        if i != min {
            nums.swap(i, min);
            count += 1;
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
    selection_sort(&mut nums);
}
