use std::io::Read;

pub fn make_triangle(p1: [f64; 2], p2: [f64; 2]) -> Vec<[f64; 2]> {
    let s = [p1[0] + (p2[0] - p1[0]) / 3.0, p1[1] + (p2[1] - p1[1]) / 3.0];
    let t = [p2[0] + (p1[0] - p2[0]) / 3.0, p2[1] + (p1[1] - p2[1]) / 3.0];
    let r = std::f64::consts::PI * 60.0 / 180.0;
    let u = [
        (t[0] - s[0]) * r.cos() - (t[1] - s[1]) * r.sin() + s[0],
        (t[0] - s[0]) * r.sin() + (t[1] - s[1]) * r.cos() + s[1],
    ];
    vec![s, u, t, p2]
}

pub fn solve(n: usize) {
    let mut points = vec![[0.0, 0.0], [100.0, 0.0]];
    for _ in 0..n {
        let mut new_points = vec![points[0]];
        for i in 1..points.len() {
            let mut tmp = make_triangle(points[i - 1], points[i]);
            new_points.append(&mut tmp);
        }
        points = new_points;
    }
    for p in points {
        println!("{:.8} {:.8}", p[0], p[1]);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    solve(buf.trim_end().parse::<usize>().unwrap());
}
