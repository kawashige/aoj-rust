use std::collections::HashSet;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn dfs(
    lists: &Vec<Vec<usize>>,
    times: &mut Vec<[usize; 2]>,
    seen: &mut HashSet<usize>,
    i: usize,
    time: &mut usize,
) {
    if times[i][0] != 0 {
        return;
    }
    times[i][0] = *time;
    *time += 1;
    for j in 2..lists[i].len() {
        dfs(lists, times, seen, lists[i][j] - 1, time);
    }
    times[i][1] = *time;
    *time += 1;
}

fn solve(lists: Vec<Vec<usize>>) {
    let mut seen = HashSet::new();
    let mut times = vec![[0, 0]; lists.len()];
    let mut time = 1;
    for i in 0..lists.len() {
        dfs(&lists, &mut times, &mut seen, i, &mut time);
    }

    for i in 0..lists.len() {
        println!("{} {} {}", i + 1, times[i][0], times[i][1])
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
