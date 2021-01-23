use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn dikstra(m: Vec<Vec<i32>>) {
    let mut d = vec![std::i32::MAX; m.len()];
    let mut color = vec![0; m.len()];
    d[0] = 0;
    color[0] = 1;
    loop {
        let mut minv = std::i32::MAX;
        let mut u = -1;
        for i in 0..m.len() {
            if d[i] < minv && color[i] != 2 {
                u = i as i32;
                minv = d[i];
            }
        }

        if u == -1 {
            break;
        }

        color[u as usize] = 2;
        for v in 0..m.len() {
            if color[v] != 2 && m[u as usize][v] != std::i32::MAX {
                if d[u as usize] + m[u as usize][v] < d[v] {
                    d[v] = d[u as usize] + m[u as usize][v];
                    color[v] = 1;
                }
            }
        }
    }

    for (i, n) in d.iter().enumerate() {
        println!("{} {}", i, n);
    }
}

fn main() {
    let n: usize = read();
    let mut matrix = vec![vec![std::i32::MAX; n]; n];
    (0..n).for_each(|_| {
        let s: String = read();
        let mut splitted = s.split(" ");
        let i: usize = splitted.next().unwrap().parse().unwrap();
        let k: usize = splitted.next().unwrap().parse().unwrap();
        for _ in 0..k {
            let j: usize = splitted.next().unwrap().parse().unwrap();
            let w: i32 = splitted.next().unwrap().parse().unwrap();
            matrix[i][j] = w;
        }
    });
    dikstra(matrix);
}
