use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn bfs(lists: &Vec<Vec<usize>>, distances: &mut Vec<i32>, nodes: Vec<usize>, d: i32) {
    let mut next = Vec::new();
    for i in nodes {
        if distances[i] != -1 {
            continue;
        }
        distances[i] = d;
        for j in 2..lists[i].len() {
            if distances[lists[i][j] - 1] == -1 {
                next.push(lists[i][j] - 1);
            }
        }
    }
    if !next.is_empty() {
        bfs(lists, distances, next, d + 1);
    }
}

fn solve(lists: Vec<Vec<usize>>) {
    let next = vec![0];
    let mut distances = vec![-1; lists.len()];
    bfs(&lists, &mut distances, next, 0);

    for i in 0..lists.len() {
        println!("{} {}", i + 1, distances[i])
    }
}

fn main() {
    let n: usize = read();
    let lists = (0..n)
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(lists);
}
