use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn max_heapify(nums: &mut Vec<i32>, i: usize) {
    let left = (i + 1) * 2 - 1;
    let right = (i + 1) * 2;
    let mut largest = i;
    if left < nums.len() && nums[largest] < nums[left] {
        largest = left;
    }
    if right < nums.len() && nums[largest] < nums[right] {
        largest = right;
    }
    if largest != i {
        nums.swap(i, largest);
        max_heapify(nums, largest);
    }
}

fn solve(mut nums: Vec<i32>) {
    for i in (0..=(nums.len() / 2)).rev() {
        max_heapify(&mut nums, i);
    }

    println!(
        " {}",
        nums.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let _: usize = read();
    let nums: Vec<i32> = read::<String>()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    solve(nums);
}
