use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    node: usize,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dikstra(adjacent_list: &Vec<Vec<Edge>>) {
    let mut heap = BinaryHeap::new();
    let mut d = vec![std::usize::MAX; adjacent_list.len()];
    let mut used = vec![false; adjacent_list.len()];
    d[0] = 0;
    heap.push(Edge { node: 0, cost: 0 });

    while let Some(Edge { node, cost }) = heap.pop() {
        if used[node] {
            continue;
        }

        used[node] = true;
        for edge in &adjacent_list[node] {
            let next = Edge {
                node: edge.node,
                cost: cost + edge.cost,
            };
            if next.cost < d[edge.node] {
                heap.push(next);
                d[edge.node] = next.cost;
            }
        }
    }

    for (i, n) in d.iter().enumerate() {
        println!("{} {}", i, n);
    }
}

fn main() {
    let n: usize = read();
    let adjacent_list = (0..n)
        .map(|_| {
            let s: String = read();
            let mut splitted = s.split(" ");
            let _: usize = splitted.next().unwrap().parse().unwrap();
            let i: usize = splitted.next().unwrap().parse().unwrap();
            (0..i)
                .map(|_| {
                    let node: usize = splitted.next().unwrap().parse().unwrap();
                    let cost: usize = splitted.next().unwrap().parse().unwrap();
                    Edge { node, cost }
                })
                .collect()
        })
        .collect();
    dikstra(&adjacent_list);
}
