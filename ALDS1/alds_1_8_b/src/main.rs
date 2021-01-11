use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[derive(Default)]
struct BinarySearchTree {
    nodes: Vec<Node>,
}

#[allow(dead_code)]
struct Node {
    val: i32,
    parent: i32,
    left: i32,
    right: i32,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, n: i32) {
        let mut y = -1;
        let mut x = if self.nodes.is_empty() { -1 } else { 0 };
        while x != -1 {
            y = x;
            if n < self.nodes[x as usize].val {
                x = self.nodes[x as usize].left;
            } else {
                x = self.nodes[x as usize].right;
            }
        }
        self.nodes.push(Node {
            val: n,
            parent: y,
            left: -1,
            right: -1,
        });

        if y != -1 {
            if n < self.nodes[y as usize].val {
                self.nodes[y as usize].left = self.nodes.len() as i32 - 1;
            } else {
                self.nodes[y as usize].right = self.nodes.len() as i32 - 1;
            }
        }
    }

    pub fn find(&self, n: i32) {
        if self.nodes.is_empty() {
            println!("no");
            return;
        }
        let mut x = 0;
        while x != -1 {
            if self.nodes[x as usize].val == n {
                println!("yes");
                return;
            } else if self.nodes[x as usize].val < n {
                x = self.nodes[x as usize].right;
            } else {
                x = self.nodes[x as usize].left;
            }
        }
        println!("no");
    }

    pub fn print(&self) {
        if self.nodes.is_empty() {
            return;
        }
        Self::inorder_traversal(&self.nodes, 0);
        print!("\n");

        Self::preorder_traversal(&self.nodes, 0);
        print!("\n");
    }

    fn preorder_traversal(nodes: &Vec<Node>, i: usize) {
        print!(" {}", nodes[i].val);
        if nodes[i].left != -1 {
            Self::preorder_traversal(nodes, nodes[i].left as usize);
        }
        if nodes[i].right != -1 {
            Self::preorder_traversal(nodes, nodes[i].right as usize);
        }
    }

    fn inorder_traversal(nodes: &Vec<Node>, i: usize) {
        if nodes[i].left != -1 {
            Self::inorder_traversal(nodes, nodes[i].left as usize);
        }
        print!(" {}", nodes[i].val);
        if nodes[i].right != -1 {
            Self::inorder_traversal(nodes, nodes[i].right as usize);
        }
    }
}

pub enum Instruction {
    Print,
    Insert(i32),
    Find(i32),
}

pub fn solve(instructions: Vec<Instruction>) {
    let mut tree = BinarySearchTree::new();
    for i in instructions {
        match i {
            Instruction::Print => tree.print(),
            Instruction::Insert(n) => tree.insert(n),
            Instruction::Find(n) => tree.find(n),
        }
    }
}

fn main() {
    let n: usize = read();
    let instructions = (0..n)
        .map(|_| {
            let i: String = read();
            if i == "print" {
                Instruction::Print
            } else if i.starts_with("insert") {
                Instruction::Insert(i.replace("insert ", "").trim_end().parse::<i32>().unwrap())
            } else {
                Instruction::Find(i.replace("find ", "").trim_end().parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Instruction>>();
    solve(instructions);
}
