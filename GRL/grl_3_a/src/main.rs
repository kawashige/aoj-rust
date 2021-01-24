use std::{collections::BTreeSet, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn dfs(
    g: &Vec<Vec<usize>>,
    parent: &mut Vec<usize>,
    prenum: &mut Vec<usize>,
    lowest: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    current: usize,
    prev: usize,
    timer: &mut usize,
) {
    prenum[current] = *timer;
    lowest[current] = *timer;
    *timer += 1;

    visited[current] = true;

    for i in 0..g[current].len() {
        let next = g[current][i];
        if !visited[next] {
            parent[next] = current;
            dfs(g, parent, prenum, lowest, visited, next, current, timer);
            lowest[current] = std::cmp::min(lowest[current], lowest[next]);
        } else if next != prev {
            lowest[current] = std::cmp::min(lowest[current], prenum[next]);
        }
    }
}

fn art_points(g: Vec<Vec<usize>>) {
    let mut visited = vec![false; g.len()];
    let mut parent = vec![0; g.len()];
    let mut prenum = vec![0; g.len()];
    let mut lowest = vec![0; g.len()];
    let mut timer = 1;

    dfs(
        &g,
        &mut parent,
        &mut prenum,
        &mut lowest,
        &mut visited,
        0,
        std::usize::MAX,
        &mut timer,
    );

    let mut np = 0;
    let mut ap = BTreeSet::new();
    for i in 1..g.len() {
        let p = parent[i];
        if p == 0 {
            np += 1;
        } else if prenum[p] <= lowest[i] {
            ap.insert(p);
        }
    }
    if 1 < np {
        ap.insert(0);
    }
    for v in ap {
        println!("{}", v);
    }
}

fn main() {
    let g = {
        let n_m = read::<String>()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut g: Vec<Vec<usize>> = vec![vec![]; n_m[0]];
        (0..n_m[1]).for_each(|_| {
            let s: String = read();
            let v = s
                .split(" ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            g[v[0]].push(v[1]);
            g[v[1]].push(v[0]);
        });
        g
    };
    art_points(g);
}
