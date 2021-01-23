use std::{collections::HashMap, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn sort(n: usize, edges: Vec<Vec<usize>>) {
    let mut result = Vec::new();
    let mut in_count = vec![0; n];
    let mut direction_map = HashMap::new();
    for i in 0..edges.len() {
        in_count[edges[i][1]] += 1;
        (*direction_map.entry(edges[i][0]).or_insert(Vec::new())).push(edges[i][1]);
    }
    while result.len() < n {
        for i in 0..in_count.len() {
            if in_count[i] == -1 {
                continue;
            }
            if in_count[i] == 0 {
                result.push(i);
                if let Some(v) = direction_map.get(&i) {
                    for j in v {
                        in_count[*j] -= 1;
                    }
                }
                in_count[i] = -1;
            }
        }
    }

    for i in result {
        println!("{}", i);
    }
}

fn main() {
    let (n, edges) = {
        let n_m = read::<String>()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let edges = (0..n_m[1])
            .map(|_| {
                let s: String = read();
                s.split(" ")
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        (n_m[0], edges)
    };
    sort(n, edges);
}
