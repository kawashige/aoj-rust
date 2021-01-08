use std::io::Read;

pub fn solve(childrens: Vec<Vec<usize>>) {
    let mut parent = vec![-1; childrens.len()];
    let mut depths = vec![0; childrens.len()];

    for i in 0..childrens.len() {
        for j in &childrens[i] {
            parent[*j] = i as i32;
        }
    }

    let p = (0..parent.len()).find(|i| parent[*i] == -1).unwrap();
    let mut next = childrens[p].clone();
    let mut depth = 1;
    while !next.is_empty() {
        let mut new_next = Vec::new();
        for n in next {
            depths[n] = depth;
            for c in &childrens[n] {
                new_next.push(*c);
            }
        }
        depth += 1;
        next = new_next;
    }

    for i in 0..childrens.len() {
        let node_type = if parent[i] == -1 {
            "root"
        } else if childrens[i].is_empty() {
            "leaf"
        } else {
            "internal node"
        };

        println!(
            "node {}: parent = {}, depth = {}, {}, [{}]",
            i,
            parent[i],
            depths[i],
            node_type,
            childrens[i]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let mut nodes = iter
        .filter(|l| !l.is_empty())
        .map(|i| {
            let mut splitted = i.split(' ');
            let id = splitted.next().unwrap().parse::<usize>().unwrap();
            let _ = splitted.next();
            let children = splitted
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (id, children)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    nodes.sort_by_key(|n| n.0);
    solve(nodes.into_iter().map(|n| n.1).collect());
}
