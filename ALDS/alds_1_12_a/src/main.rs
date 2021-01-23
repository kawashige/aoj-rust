use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn prim(m: Vec<Vec<i32>>) -> i32 {
    let l = m.len();
    let mut color = vec![0; l];
    let mut d = vec![std::i32::MAX; l];
    let mut p = vec![-1_i32; l];
    d[0] = 0;
    p[0] = -1;

    loop {
        let mut min_cost = std::i32::MAX;
        let mut u: i32 = -1;
        for i in 0..l {
            if color[i] != 2 && d[i] < min_cost {
                min_cost = d[i];
                u = i as i32;
            }
        }

        if u == -1 {
            break;
        }

        color[u as usize] = 2;

        for i in 0..l {
            if color[i] != 2 && m[u as usize][i] != std::i32::MAX {
                if m[u as usize][i] < d[i] {
                    d[i] = m[u as usize][i];
                    p[i] = u;
                    color[i] = 1;
                }
            }
        }
    }
    let mut sum = 0;
    for i in 0..l {
        if p[i] != -1 {
            sum += m[i][p[i] as usize];
        }
    }
    sum
}

fn solve(adjacent_lists: Vec<Vec<i32>>) {
    let min_cost = prim(adjacent_lists);
    println!("{}", min_cost);
}

fn main() {
    let n: usize = read();
    let adjacent_lists = (0..n)
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| {
                    let n = l.parse::<i32>().unwrap();
                    if n == -1 {
                        std::i32::MAX
                    } else {
                        n
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect();
    solve(adjacent_lists);
}
