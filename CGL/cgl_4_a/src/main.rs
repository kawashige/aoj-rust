use std::ops;
use std::str::FromStr;

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

    pub fn intersct(&self, other: &Segment) -> bool {
        Point::intersect(&self.p1, &self.p2, &other.p1, &other.p2)
    }

    pub fn project(&self, p: Point) -> Point {
        let base = self.p1 - self.p2;
        let r = base.dot(p - self.p1) / base.norm();
        self.p1 + base * r
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn norm(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn dot(&self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Point) -> i32 {
        self.x * other.y - self.y * other.x
    }

    pub fn ccw(p0: &Point, p1: &Point, p2: &Point) -> i32 {
        let a = p1 - p0;
        let b = p2 - p0;
        if a.cross(b) > 0 {
            1
        } else if a.cross(b) < 0 {
            -1
        } else if a.dot(b) < 0 {
            2
        } else if a.norm() < b.norm() {
            -2
        } else {
            0
        }
    }

    pub fn intersect(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
        Self::ccw(p1, p2, p3) * Self::ccw(p1, p2, p4) <= 0
            && Self::ccw(p3, p4, p1) * Self::ccw(p3, p4, p2) <= 0
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: &'b Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: &'b Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn andrew_scan(mut p: Vec<Point>) -> Vec<Point> {
    if p.len() < 3 {
        return p;
    }

    p.sort_unstable_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    let mut u = Vec::new();
    let mut l = Vec::new();

    u.push(p[0]);
    u.push(p[1]);
    l.push(p[p.len() - 1]);
    l.push(p[p.len() - 2]);

    for i in 2..p.len() {
        let mut n = u.len();
        while n >= 2 && Point::ccw(&u[n - 2], &u[n - 1], &p[i]) == 1 {
            u.pop();
            n -= 1;
        }
        u.push(p[i])
    }

    for i in (0..(p.len() - 2)).rev() {
        let mut n = l.len();
        while n >= 2 && Point::ccw(&l[n - 2], &l[n - 1], &p[i]) == 1 {
            l.pop();
            n -= 1;
        }
        l.push(p[i])
    }
    l.reverse();

    for i in (1..(u.len() - 1)).rev() {
        l.push(u[i])
    }

    l
}

fn solve(p: Vec<Point>) {
    let mut result = andrew_scan(p);
    println!("{}", result.len());
    let i = result
        .iter()
        .enumerate()
        .min_by_key(|(_, r)| (r.y, r.x))
        .unwrap()
        .0;

    result.rotate_left(i);
    for p in result {
        println!("{} {}", p.x, p.y);
    }
}

fn main() {
    let n: usize = read();
    let p = (0..n)
        .map(|_| {
            let v = read::<String>()
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Point::new(v[0], v[1])
        })
        .collect();
    solve(p);
}
