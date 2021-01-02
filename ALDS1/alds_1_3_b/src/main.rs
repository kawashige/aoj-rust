use std::collections::VecDeque;
use std::io::Read;

pub fn roundrobin(q: i32, processes: &mut VecDeque<(&str, i32)>) {
    let mut time = 0;
    while let Some(mut p) = processes.pop_front() {
        if p.1 <= q {
            time += p.1;
            println!("{} {}", p.0, time);
        } else {
            time += q;
            p.1 -= q;
            processes.push_back(p);
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let mut first = iter.next().unwrap().split(' ');
    let _n = first.next().unwrap();
    let q = first.next().unwrap().parse::<i32>().unwrap();
    let mut processes = iter
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            let mut splitted = l.split(' ');
            let name = splitted.next().unwrap();
            let time = splitted.next().unwrap().parse::<i32>().unwrap();
            Some((name, time))
        })
        .collect::<VecDeque<(&str, i32)>>();
    roundrobin(q, &mut processes);
}
