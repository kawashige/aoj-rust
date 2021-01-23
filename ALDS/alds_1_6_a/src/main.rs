use std::io::Read;

pub fn countint_sort(input: &Vec<i16>, output: &mut Vec<i16>, k: i16) {
    let mut count = vec![0; k as usize + 1];
    for i in input {
        count[*i as usize] += 1;
    }
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }
    for i in (0..input.len()).rev() {
        output[count[input[i] as usize] - 1] = input[i];
        count[input[i] as usize] -= 1;
    }
}

pub fn solve(input: Vec<i16>) {
    let mut output = vec![0; input.len()];
    countint_sort(&input, &mut output, 10000);
    println!(
        "{}",
        output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
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
            .map(|i| i.parse::<i16>().unwrap())
            .collect()
    };
    solve(nums);
}
