use std::ops;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

struct Circle {
    c: Point,
    r: f64,
}

impl Circle {
    pub fn new(c: Point, r: f64) -> Self {
        Self { c, r }
    }

    pub fn get_cross_points(&self, other: Circle) -> (Point, Point) {
        let d = (self.c - other.c).abs();
        let a = ((self.r * self.r + d * d - other.r * other.r) / (2.0 * self.r * d)).acos();
        let t = (other.c - self.c).arg();
        (
            self.c + Point::polar(self.r, t + a),
            self.c + Point::polar(self.r, t - a),
        )
    }
}

struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn get_distance(&self, other: &Segment) -> f64 {
        if self.intersct(&other) {
            return 0.0;
        }
        self.get_distance_sp(&other.p1)
            .min(self.get_distance_sp(&other.p2))
            .min(other.get_distance_sp(&self.p1))
            .min(other.get_distance_sp(&self.p2))
    }

    pub fn get_distance_sp(&self, p: &Point) -> f64 {
        if (&self.p2 - &self.p1).dot(p - &self.p1) < 0.0 {
            (p - &self.p1).abs()
        } else if (&self.p1 - &self.p2).dot(p - &self.p2) < 0.0 {
            (p - &self.p2).abs()
        } else {
            self.get_distance_lp(p)
        }
    }

    pub fn get_distance_lp(&self, p: &Point) -> f64 {
        (&self.p2 - &self.p1).cross(p - &self.p1).abs() / (&self.p2 - &self.p1).abs()
    }

    pub fn intersct(&self, other: &Segment) -> bool {
        Point::intersect(&self.p1, &self.p2, &other.p1, &other.p2)
    }

    pub fn reflect(&self, p: Point) -> Point {
        p + (self.project(p) - p) * 2.0
    }

    pub fn project(&self, p: Point) -> Point {
        let base = self.p1 - self.p2;
        let r = base.dot(p - self.p1) / base.norm();
        self.p1 + base * r
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn abs(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn ccw(p0: &Point, p1: &Point, p2: &Point) -> i32 {
        let a = p1 - p0;
        let b = p2 - p0;
        if a.cross(b) > std::f64::EPSILON {
            1
        } else if a.cross(b) < -std::f64::EPSILON {
            -1
        } else if a.dot(b) < -std::f64::EPSILON {
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

    pub fn arg(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn polar(a: f64, r: f64) -> Self {
        Self {
            x: r.cos() * a,
            y: r.sin() * a,
        }
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

impl ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn solve(c1: Circle, c2: Circle) {
    let (x1, x2) = c1.get_cross_points(c2);
    if x1 < x2 {
        println!("{:.10} {:.10} {:.10} {:.10}", x1.x, x1.y, x2.x, x2.y);
    } else {
        println!("{:.10} {:.10} {:.10} {:.10}", x2.x, x2.y, x1.x, x1.y);
    }
}

fn main() {
    let c1 = {
        let v = read::<String>()
            .split(" ")
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Circle::new(Point::new(v[0], v[1]), v[2])
    };
    let c2 = {
        let v = read::<String>()
            .split(" ")
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Circle::new(Point::new(v[0], v[1]), v[2])
    };
    solve(c1, c2);
}
