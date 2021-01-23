use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn warshall_floyd(mut d: Vec<Vec<i32>>) {
    for k in 0..d.len() {
        for i in 0..d.len() {
            if d[i][k] == std::i32::MAX {
                continue;
            }
            for j in 0..d.len() {
                if d[k][j] == std::i32::MAX {
                    continue;
                }
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    if (0..d.len()).any(|i| d[i][i] < 0) {
        println!("NEGATIVE CYCLE");
    } else {
        for i in 0..d.len() {
            println!(
                "{}",
                d[i].iter()
                    .map(|n| if n == &(std::i32::MAX) {
                        "INF".to_string()
                    } else {
                        n.to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}

fn main() {
    let n_m = read::<String>()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut matrix = vec![vec![std::i32::MAX; n_m[0]]; n_m[0]];
    for i in 0..n_m[0] {
        matrix[i][i] = 0;
    }
    (0..n_m[1]).for_each(|_| {
        let s: String = read();
        let mut splitted = s.split(" ");
        let s: usize = splitted.next().unwrap().parse().unwrap();
        let t: usize = splitted.next().unwrap().parse().unwrap();
        let d: i32 = splitted.next().unwrap().parse().unwrap();
        matrix[s][t] = d;
    });
    warshall_floyd(matrix);
}
