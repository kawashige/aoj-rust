use std::io::Read;

pub fn exec(symbols: Vec<&str>) {
    let mut stack: Vec<i32> = Vec::new();
    for s in symbols {
        match s {
            "+" => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 + n2);
            }
            "-" => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 - n2);
            }
            "*" => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 * n2);
            }
            _ => {
                stack.push(s.parse::<i32>().unwrap());
            }
        }
    }
    println!("{}", stack.pop().unwrap());
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let symbols = buf.trim_end_matches("\n").split(" ").collect::<Vec<&str>>();
    exec(symbols);
}
