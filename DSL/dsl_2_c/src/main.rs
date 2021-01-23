use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[derive(Clone)]
struct Node {
    location: usize,
    l: usize,
    r: usize,
}

#[derive(Clone, Copy)]
struct Point {
    location: usize,
    x: i32,
    y: i32,
}

fn make_kd_tree(
    points: &mut Vec<Point>,
    tree: &mut Vec<Node>,
    np: &mut usize,
    l: usize,
    r: usize,
    depth: usize,
) -> usize {
    if r <= l {
        return std::usize::MAX;
    }
    let mid = (l + r) / 2;
    let mut part = points[l..r].to_vec();
    if depth % 2 == 0 {
        part.sort_by_key(|p| p.x);
    } else {
        part.sort_by_key(|p| p.y);
    }
    for i in l..r {
        points[i] = part[i - l];
    }

    let t = *np;
    *np += 1;
    tree[t].location = mid;
    tree[t].l = make_kd_tree(points, tree, np, l, mid, depth + 1);
    tree[t].r = make_kd_tree(points, tree, np, mid + 1, r, depth + 1);

    return t;
}

fn find(
    points: &Vec<Point>,
    tree: &Vec<Node>,
    i: usize,
    range: &Vec<i32>,
    depth: usize,
    result: &mut Vec<usize>,
) {
    let x = points[tree[i].location].x;
    let y = points[tree[i].location].y;

    if range[0] <= x && x <= range[1] && range[2] <= y && y <= range[3] {
        result.push(points[tree[i].location].location);
    }

    if depth % 2 == 0 {
        if tree[i].l != std::usize::MAX && range[0] <= x {
            find(points, tree, tree[i].l, range, depth + 1, result);
        }
        if tree[i].r != std::usize::MAX && x <= range[1] {
            find(points, tree, tree[i].r, range, depth + 1, result);
        }
    } else {
        if tree[i].l != std::usize::MAX && range[2] <= y {
            find(points, tree, tree[i].l, range, depth + 1, result);
        }
        if tree[i].r != std::usize::MAX && y <= range[3] {
            find(points, tree, tree[i].r, range, depth + 1, result);
        }
    }
}

fn solve(mut points: Vec<Point>, ranges: Vec<Vec<i32>>) {
    let l = points.len();
    let mut tree = vec![
        Node {
            location: 0,
            l: std::usize::MAX,
            r: std::usize::MAX
        };
        l
    ];
    let mut np = 0;

    let root = make_kd_tree(&mut points, &mut tree, &mut np, 0, l, 0);

    for r in ranges {
        let mut result = Vec::new();
        find(&points, &tree, root, &r, 0, &mut result);
        result.sort();
        for r in result {
            println!("{}", r);
        }
        println!("");
    }
}

fn main() {
    let n: usize = read();
    let points = (0..n)
        .map(|i| {
            let s: String = read();
            let v = s
                .split(' ')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Point {
                location: i,
                x: v[0],
                y: v[1],
            }
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
