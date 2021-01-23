use std::collections::HashSet;
use std::io::Read;

pub fn exec(ops: Vec<&str>) {
    let mut dict = HashSet::new();
    for op in ops {
        if op.is_empty() {
            continue;
        }
        if op.starts_with("insert") {
            let v = op.replace("insert ", "");
            dict.insert(v);
        } else {
            let v = op.replace("find ", "");
            if dict.contains(&v) {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _ = iter.next().unwrap();
    let ops = iter.collect();
    exec(ops);
}
