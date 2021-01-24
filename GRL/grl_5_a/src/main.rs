use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn dfs(
    g: &Vec<Vec<(usize, usize)>>,
    visited: &mut Vec<bool>,
    current: usize,
    result: &mut usize,
) -> usize {
    visited[current] = true;

    let mut depths = Vec::new();
    for i in 0..g[current].len() {
        if visited[g[current][i].0] {
            continue;
        }
        depths.push(g[current][i].1 + dfs(g, visited, g[current][i].0, result));
    }
    depths.sort_unstable();
    if depths.len() > 1 {
        *result = std::cmp::max(*result, depths[depths.len() - 2] + depths[depths.len() - 1]);
    } else if depths.len() == 1 {
        *result = std::cmp::max(*result, depths[depths.len() - 1]);
    }

    *depths.last().unwrap_or(&0)
}

fn solve(g: Vec<Vec<(usize, usize)>>) {
    let mut visited = vec![false; g.len()];
    let mut result = 0;

    dfs(&g, &mut visited, 0, &mut result);

    println!("{}", result);
}

fn main() {
    let g = {
        let n: usize = read();
        let mut g = vec![vec![]; n];
        (1..n).for_each(|_| {
            let s: String = read();
            let v = s
                .split(" ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            g[v[0]].push((v[1], v[2]));
            g[v[1]].push((v[0], v[2]));
        });
        g
    };
    solve(g);
}
