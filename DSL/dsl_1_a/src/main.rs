use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

enum Query {
    Unite((usize, usize)),
    Same((usize, usize)),
}

fn root(parents: &mut Vec<usize>, i: usize) -> usize {
    if parents[i] == i {
        return i;
    }
    parents[i] = root(parents, parents[i]);
    parents[i]
}

fn solve(n: usize, queries: Vec<Query>) {
    let mut parents = (0..n).collect::<Vec<usize>>();

    for q in queries {
        match q {
            Query::Unite((n1, n2)) => {
                let r1 = root(&mut parents, n1);
                let r2 = root(&mut parents, n2);
                if r1 == r2 {
                    continue;
                }
                parents[r1] = r2;
            }
            Query::Same((n1, n2)) => {
                let r1 = root(&mut parents, n1);
                let r2 = root(&mut parents, n2);
                if r1 == r2 {
                    println!("1");
                } else {
                    println!("0");
                }
            }
        }
    }
}

fn main() {
    let n_q = read::<String>()
        .split(' ')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let queries = (0..n_q[1])
        .map(|_| {
            let s: String = read();
            let v = s
                .split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if v[0] == 0 {
                Query::Unite((v[1], v[2]))
            } else {
                Query::Same((v[1], v[2]))
            }
        })
        .collect();
    solve(n_q[0], queries);
}
