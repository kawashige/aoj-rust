use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(points: Vec<Vec<i32>>, ranges: Vec<Vec<i32>>) {
    let mut x_map = BTreeMap::new();
    let mut y_map = BTreeMap::new();
    for (i, p) in points.iter().enumerate() {
        (*x_map.entry(p[0]).or_insert(BTreeSet::new())).insert(i);
        (*y_map.entry(p[1]).or_insert(BTreeSet::new())).insert(i);
    }

    for r in ranges {
        let x_candidates = x_map
            .range(r[0]..=r[1])
            .map(|(_, v)| v)
            .fold(BTreeSet::new(), |acc, s| acc.union(s).cloned().collect());
        let y_candidates = y_map
            .range(r[2]..=r[3])
            .map(|(_, v)| v)
            .fold(BTreeSet::new(), |acc, s| acc.union(s).cloned().collect());

        for p in x_candidates.intersection(&y_candidates) {
            println!("{}", p);
        }
        println!("");
    }
}

fn main() {
    let n: usize = read();
    let points = (0..n)
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let q: usize = read();
    let ranges = (0..q)
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    solve(points, ranges);
}
