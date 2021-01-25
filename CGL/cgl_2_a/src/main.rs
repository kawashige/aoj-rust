use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(points: Vec<Vec<i32>>) {
    for p in points {
        let p1 = [p[0] - p[2], p[1] - p[3]];
        let p2 = [p[4] - p[6], p[5] - p[7]];
        if p1[0] * p2[0] + p1[1] * p2[1] == 0 {
            println!("1");
        } else if p1[0] * p2[1] - p1[1] * p2[0] == 0 {
            println!("2");
        } else {
            println!("0");
        }
    }
}

fn main() {
    let n: usize = read();
    let points = (0..n)
        .map(|_| {
            let s: String = read();
            s.split(' ').map(|l| l.parse().unwrap()).collect()
        })
        .collect();
    solve(points);
}
