use std::str::FromStr;

enum Instruction {
    Insert(i32),
    Extract,
}

impl From<String> for Instruction {
    fn from(src: String) -> Self {
        if src.starts_with("insert ") {
            Instruction::Insert(src.replace("insert ", "").parse::<i32>().unwrap())
        } else {
            Instruction::Extract
        }
    }
}

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn upward_heapify(nums: &mut Vec<i32>, i: usize) {
    if i == 0 {
        return;
    }
    let parent = (i + 1) / 2 - 1;
    if nums[parent] < nums[i] {
        nums.swap(i, parent);
        if 0 < parent {
            upward_heapify(nums, parent);
        }
    }
}

fn downward_heapify(nums: &mut Vec<i32>, i: usize) {
    let left = (i + 1) * 2 - 1;
    let right = (i + 1) * 2;
    let mut largest = i;
    if left < nums.len() && nums[largest] < nums[left] {
        largest = left;
    }
    if right < nums.len() && nums[largest] < nums[right] {
        largest = right;
    }
    if largest != i {
        nums.swap(i, largest);
        downward_heapify(nums, largest);
    }
}

fn solve(instructions: Vec<Instruction>) {
    let mut nums = Vec::new();
    for i in instructions {
        match i {
            Instruction::Insert(n) => {
                nums.push(n);
                let i = nums.len() - 1;
                upward_heapify(&mut nums, i);
            }
            Instruction::Extract => {
                let max = nums[0];
                println!("{}", max);
                let last = nums.pop().unwrap();
                if !nums.is_empty() {
                    nums[0] = last;
                    downward_heapify(&mut nums, 0);
                }
            }
        }
    }
}

fn main() {
    let mut instructions = Vec::new();
    loop {
        let s: String = read();
        if s == "end".to_string() {
            break;
        }
        instructions.push(s.into());
    }
    solve(instructions);
}
