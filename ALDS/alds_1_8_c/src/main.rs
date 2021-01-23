use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

#[derive(Default)]
struct BinarySearchTree {
    nodes: HashMap<i32, RefCell<Node>>,
    root: i32,
    next_id: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
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
            let node = self.nodes.get(&x).unwrap().borrow();
            if n < node.val {
                x = node.left;
            } else {
                x = node.right;
            }
        }
        if self.nodes.is_empty() {
            self.root = self.next_id;
        }
        self.nodes.insert(
            self.next_id,
            RefCell::new(Node {
                val: n,
                parent: y,
                left: -1,
                right: -1,
            }),
        );

        if y != -1 {
            let mut node = self.nodes.get_mut(&y).unwrap().borrow_mut();
            if n < node.val {
                node.left = self.next_id;
            } else {
                node.right = self.next_id;
            }
        }

        self.next_id += 1;
    }

    pub fn find_and_print(&self, n: i32) {
        if self.find(n) == -1 {
            println!("no");
        } else {
            println!("yes");
        }
    }

    pub fn find(&self, n: i32) -> i32 {
        if self.nodes.is_empty() {
            return -1;
        }
        let mut x = 0;
        while x != -1 {
            let node = self.nodes.get(&x).unwrap().borrow();
            if node.val == n {
                return x;
            } else if node.val < n {
                x = node.right;
            } else {
                x = node.left;
            }
        }
        -1
    }

    pub fn delete(&mut self, n: i32) {
        let id = self.find(n);
        if id == -1 {
            return;
        }
        self.delete_id(id);
    }

    pub fn delete_id(&mut self, id: i32) {
        let mut delete_id = -1;
        {
            let mut node = self.nodes.get(&id).unwrap().borrow_mut();
            if node.right == -1 && node.left == -1 {
                let mut parent = self.nodes.get(&node.parent).unwrap().borrow_mut();
                if parent.left == id {
                    parent.left = -1;
                } else {
                    parent.right = -1;
                }
            } else if node.right == -1 {
                let mut left = self.nodes.get(&node.left).unwrap().borrow_mut();
                left.parent = node.parent;
                let mut parent = self.nodes.get(&node.parent).unwrap().borrow_mut();
                if parent.left == id {
                    parent.left = node.left;
                } else {
                    parent.right = node.left;
                }
            } else if node.left == -1 {
                let mut right = self.nodes.get(&node.right).unwrap().borrow_mut();
                right.parent = node.parent;
                let mut parent = self.nodes.get(&node.parent).unwrap().borrow_mut();
                if parent.right == id {
                    parent.right = node.right;
                } else {
                    parent.left = node.right;
                }
            } else {
                let mut find_node = self.nodes.get(&node.right).unwrap().borrow();
                delete_id = node.right;
                while find_node.left != -1 {
                    delete_id = find_node.left;
                    find_node = self.nodes.get(&find_node.left).unwrap().borrow();
                }
                node.val = find_node.val;
            }
        }
        if delete_id != -1 {
            self.delete_id(delete_id);
        } else {
            self.nodes.remove(&id);
        }
    }

    pub fn print(&self) {
        if self.nodes.is_empty() {
            return;
        }
        Self::inorder_traversal(&self.nodes, self.root);
        print!("\n");

        Self::preorder_traversal(&self.nodes, self.root);
        print!("\n");
    }

    fn preorder_traversal(nodes: &HashMap<i32, RefCell<Node>>, i: i32) {
        let node = nodes.get(&i).unwrap().borrow();
        print!(" {}", node.val);
        if node.left != -1 {
            Self::preorder_traversal(nodes, node.left);
        }
        if node.right != -1 {
            Self::preorder_traversal(nodes, node.right);
        }
    }

    fn inorder_traversal(nodes: &HashMap<i32, RefCell<Node>>, i: i32) {
        let node = nodes.get(&i).unwrap().borrow();
        if node.left != -1 {
            Self::inorder_traversal(nodes, node.left);
        }
        print!(" {}", node.val);
        if node.right != -1 {
            Self::inorder_traversal(nodes, node.right);
        }
    }
}

pub enum Instruction {
    Print,
    Insert(i32),
    Find(i32),
    Delete(i32),
}

pub fn solve(instructions: Vec<Instruction>) {
    let mut tree = BinarySearchTree::new();
    for i in instructions {
        match i {
            Instruction::Print => tree.print(),
            Instruction::Insert(n) => tree.insert(n),
            Instruction::Find(n) => tree.find_and_print(n),
            Instruction::Delete(n) => tree.delete(n),
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
            } else if i.starts_with("find") {
                Instruction::Find(i.replace("find ", "").trim_end().parse::<i32>().unwrap())
            } else {
                Instruction::Delete(i.replace("delete ", "").trim_end().parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<Instruction>>();
    solve(instructions);
}
