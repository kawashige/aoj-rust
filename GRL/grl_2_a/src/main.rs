use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

struct DisjointSet {
    parents: Vec<usize>,
}

impl DisjointSet {
    pub fn new(len: usize) -> Self {
        Self {
            parents: (0..len).collect(),
        }
    }

    pub fn root(&mut self, target: usize) -> usize {
        if self.parents[target] == target {
            return target;
        }
        self.parents[target] = self.root(self.parents[target]);
        self.parents[target]
    }

    pub fn union(&mut self, source: usize, target: usize) {
        let source_root = self.root(source);
        let target_root = self.root(target);
        if source_root != target_root {
            self.parents[target_root] = source_root;
        }
    }

    pub fn same(&mut self, source: usize, target: usize) -> bool {
        self.root(source) == self.root(target)
    }
}

fn kraskal(n: usize, mut edges: Vec<Vec<usize>>) -> usize {
    let mut cost = 0;

    edges.sort_by_key(|e| e[2]);
    let mut disjoint_set = DisjointSet::new(n);

    for i in 0..edges.len() {
        if !disjoint_set.same(edges[i][0], edges[i][1]) {
            cost += edges[i][2];
            disjoint_set.union(edges[i][0], edges[i][1]);
        }
    }

    cost
}

fn solve(n: usize, edges: Vec<Vec<usize>>) {
    let min_cost = kraskal(n, edges);
    println!("{}", min_cost);
}

fn main() {
    let n_m: Vec<usize> = read::<String>()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let edges = (0..n_m[1])
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(n_m[0], edges);
}
