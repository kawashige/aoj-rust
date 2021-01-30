use std::str::FromStr;
use std::{cmp::Ordering, collections::BTreeSet};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Kind {
    Bottom(i32),
    LR(i32, i32),
    Top(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct EndPoint {
    y: i32,
    kind: Kind,
}

impl EndPoint {
    pub fn new(y: i32, kind: Kind) -> Self {
        Self { y, kind }
    }
}

impl PartialOrd for EndPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for EndPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        let first = self.y.cmp(&other.y);
        if first == Ordering::Equal {
            self.kind.cmp(&other.kind)
        } else {
            first
        }
    }
}

fn solve(s: Vec<Segment>) {
    let mut ep = Vec::with_capacity(s.len() * 2);
    let n = s.len();

    for i in 0..n {
        let mut p1 = s[i].p1;
        let mut p2 = s[i].p2;
        if s[i].p1.y == s[i].p2.y && s[i].p1.x > s[i].p2.x {
            std::mem::swap(&mut p1, &mut p2);
        } else if s[i].p1.y > s[i].p2.y {
            std::mem::swap(&mut p1, &mut p2);
        }

        if p1.y == p2.y {
            ep.push(EndPoint::new(p1.y, Kind::LR(p1.x, p2.x)));
        } else {
            ep.push(EndPoint::new(p1.y, Kind::Bottom(p1.x)));
            ep.push(EndPoint::new(p2.y, Kind::Top(p2.x)));
        }
    }

    ep.sort();

    let mut bt = BTreeSet::new();
    let mut count = 0;
    for p in ep {
        match p.kind {
            Kind::Top(x) => {
                bt.remove(&x);
            }
            Kind::Bottom(x) => {
                bt.insert(x);
            }
            Kind::LR(xl, xr) => count += bt.range(xl..=xr).count(),
        }
    }
    println!("{}", count);
}

fn main() {
    let n: usize = read();
    let segments = (0..n)
        .map(|_| {
            let v = read::<String>()
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Segment::new(Point::new(v[0], v[1]), Point::new(v[2], v[3]))
        })
        .collect();
    solve(segments);
}
