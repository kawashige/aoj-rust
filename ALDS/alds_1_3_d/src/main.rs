use std::io::Read;

pub fn calc(chars: Vec<char>) {
    let mut area = 0;
    let mut areas: Vec<(usize, usize)> = Vec::new();
    let mut stack = Vec::new();
    for (i, c) in chars.into_iter().enumerate() {
        if c == '\\' {
            stack.push(i);
        } else if c == '/' && !stack.is_empty() {
            let j = stack.pop().unwrap();
            area += i - j;
            let mut tmp = i - j;
            while !areas.is_empty() && j < areas.last().unwrap().0 {
                tmp += areas.pop().unwrap().1;
            }
            areas.push((j, tmp));
        }
    }

    println!("{}", area);
    if areas.is_empty() {
        println!("0");
    } else {
        println!(
            "{} {}",
            areas.len(),
            areas
                .into_iter()
                .map(|a| a.1.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    calc(buf.chars().collect());
}
