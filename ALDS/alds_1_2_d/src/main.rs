use std::io::Read;

pub fn print(nums: &Vec<i32>) {
    println!(
        "{}",
        nums.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn insertion_sort(nums: &mut Vec<i32>, g: usize, count: &mut i32) {
    for i in g..nums.len() {
        let v = nums[i];
        let mut j = i;
        while g <= j && v < nums[j - g] {
            j -= g;
            nums[j + g] = nums[j];
            *count += 1;
        }
        nums[j] = v;
    }
}

pub fn shell_sort(nums: &mut Vec<i32>) {
    let mut count = 0;
    let mut gs = vec![1];
    while gs.last().unwrap() < &nums.len() {
        gs.push(3 * gs.last().unwrap() + 1);
    }
    if 1 < gs.len() {
        gs.pop();
    }
    gs.reverse();
    for g in &gs {
        insertion_sort(nums, *g, &mut count);
    }
    println!("{}", gs.len());
    println!(
        "{}",
        gs.into_iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("{}", count);
    print(nums);
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums = iter
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    shell_sort(&mut nums);
}
