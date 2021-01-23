use std::io::Read;

pub fn print(nums: &Vec<i32>) {
    println!("{}", nums.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
}

pub fn insertion_sort(nums: &mut Vec<i32>) {
    print(nums);
    for i in 1..nums.len() {
        for j in (1..=i).rev() {
            if nums[j] < nums[j - 1] {
                nums.swap(j, j - 1);
            } else {
                break;
            }
        }
        print(nums);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums = iter.map(|i| i.parse::<i32>().unwrap()).collect::<Vec<_>>();

    insertion_sort(&mut nums);
}
