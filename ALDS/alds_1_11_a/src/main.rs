use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn solve(lists: Vec<Vec<usize>>) {
    let mut matrix = vec![vec![0; lists.len()]; lists.len()];
    for l in lists {
        for i in 2..l.len() {
            matrix[l[0] - 1][l[i] - 1] = 1;
        }
    }
    for i in 0..matrix.len() {
        println!(
            "{}",
            matrix[i]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
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
