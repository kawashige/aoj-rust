use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let n1: i32 = iter.next().unwrap().parse().unwrap();
    let n2: i32 = iter.next().unwrap().parse().unwrap();
    let mut min = std::cmp::min(n1, n2);
    let mut max_profit: i32 = n2 - n1;
    for _ in 0..(n - 2) {
        let num: i32 = iter.next().unwrap().parse().unwrap();
        if num < min {
            min = num;
        } else if max_profit < num - min {
            max_profit = num - min;
        }
    }
    println!("{}", max_profit);
}
