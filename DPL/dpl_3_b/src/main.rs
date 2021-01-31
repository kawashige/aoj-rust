use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn get_largest_rectangle(t: &Vec<usize>) -> usize {
    let mut max = 0;
    let mut stack = Vec::new();

    for i in 0..t.len() {
        if stack.is_empty() {
            stack.push((i, t[i]));
        } else {
            if stack[stack.len() - 1].1 < t[i] {
                stack.push((i, t[i]));
            } else if stack[stack.len() - 1].1 > t[i] {
                let mut target = i;
                while !stack.is_empty() && stack[stack.len() - 1].1 >= t[i] {
                    let top = stack.pop().unwrap();
                    max = std::cmp::max(max, top.1 * (i - top.0));
                    target = top.0;
                }
                stack.push((target, t[i]));
            }
        }
    }
    max
}

fn solve(nums: Vec<Vec<usize>>) {
    let mut t = vec![vec![0; nums[0].len() + 1]; nums.len()];
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            if nums[i][j] == 1 {
                t[i][j] = 0;
            } else {
                t[i][j] = if i > 0 { t[i - 1][j] + 1 } else { 1 };
            }
        }
    }

    let mut max = 0;
    for i in 0..t.len() {
        max = std::cmp::max(max, get_largest_rectangle(&t[i]))
    }
    println!("{}", max);
}

fn main() {
    let n: usize = {
        let nums = read::<String>()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        nums[0]
    };
    let nums = (0..n)
        .map(|_| {
            read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(nums);
}
