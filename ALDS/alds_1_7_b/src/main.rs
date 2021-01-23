use std::io::Read;

pub fn solve(childrens: Vec<Vec<usize>>) {
    let mut parent = vec![-1; childrens.len()];
    let mut depths = vec![0; childrens.len()];
    let mut height = vec![0; childrens.len()];

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

    for i in (0..childrens.len()).filter(|i| childrens[*i].is_empty()) {
        let mut p = parent[i];
        let mut h = 1;
        while p != -1 {
            if height[p as usize] < h {
                height[p as usize] = h;
            }
            h += 1;
            p = parent[p as usize];
        }
    }

    for i in 0..childrens.len() {
        let node_type = if parent[i] == -1 {
            "root"
        } else if childrens[i].is_empty() {
            "leaf"
        } else {
            "internal node"
        };
        let sigbling = (0..parent.len() as i32)
            .find(|j| *j as usize != i && parent[*j as usize] == parent[i])
            .unwrap_or(-1);
        let degree = parent.iter().filter(|j| **j == i as i32).count();

        println!(
            "node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}",
            i, parent[i], sigbling, degree, depths[i], height[i], node_type,
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
            let children = splitted
                .filter(|s| s != &"-1")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (id, children)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    nodes.sort_by_key(|n| n.0);
    solve(nodes.into_iter().map(|n| n.1).collect());
}
