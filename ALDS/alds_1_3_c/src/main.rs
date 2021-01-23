use std::collections::VecDeque;
use std::io::Read;

pub fn exec(ops: Vec<&str>) {
    let mut list = VecDeque::new();

    for op in ops {
        if op.is_empty() {
            continue;
        }
        if op.starts_with("deleteFirst") {
            list.pop_front();
        } else if op.starts_with("deleteLast") {
            list.pop_back();
        } else if op.starts_with("delete") {
            let n = op.replace("delete ", "").parse::<i32>().unwrap();
            if let Some(p) = (0..list.len()).find(|i| list[*i] == n) {
                list.remove(p);
            }
        } else {
            let n = op.replace("insert ", "").parse::<i32>().unwrap();
            list.push_front(n);
        }
    }

    println!(
        "{}",
        list.into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let ops = iter.collect::<Vec<&str>>();
    exec(ops);
}
