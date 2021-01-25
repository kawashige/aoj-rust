use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(p: Vec<i32>, points: Vec<Vec<i32>>) {
    let v1 = [p[2] - p[0], p[3] - p[1]];
    let v1_norm = v1[0] * v1[0] + v1[1] * v1[1];
    for point in points {
        let v2 = [point[0] - p[0], point[1] - p[1]];
        let r = (v1[0] * v2[0] + v1[1] * v2[1]) as f64 / v1_norm as f64;
        println!(
            "{:.10} {:.10}",
            p[0] as f64 + v1[0] as f64 * r,
            p[1] as f64 + v1[1] as f64 * r
        );
    }
}

fn main() {
    let p: Vec<i32> = read::<String>()
        .split(' ')
        .map(|l| l.parse().unwrap())
        .collect();
    let n: usize = read();
    let points = (0..n)
        .map(|_| {
            let s: String = read();
            s.split(' ').map(|l| l.parse().unwrap()).collect()
        })
        .collect();
    solve(p, points);
}
