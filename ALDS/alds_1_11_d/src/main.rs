use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn root(parents: &mut Vec<usize>, i: usize) -> usize {
    if parents[i] == i {
        return i;
    }
    parents[i] = root(parents, parents[i]);
    parents[i]
}

fn solve(n: usize, relations: Vec<Vec<usize>>, questions: Vec<Vec<usize>>) {
    let mut parents = (0..n).collect::<Vec<usize>>();

    for r in relations {
        let r0 = root(&mut parents, r[0]);
        let r1 = root(&mut parents, r[1]);
        if r0 == r1 {
            continue;
        }
        parents[r0] = r1;
    }

    for q in questions {
        let r0 = root(&mut parents, q[0]);
        let r1 = root(&mut parents, q[1]);
        if r0 == r1 {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn main() {
    let n_m = read::<String>()
        .split(' ')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let relations = (0..n_m[1])
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let q: usize = read();
    let questions = (0..q)
        .map(|_| {
            let s: String = read();
            s.split(' ')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    solve(n_m[0], relations, questions);
}
