use std::io::Read;

pub fn find_root(nodes: &Vec<Vec<i32>>) -> usize {
    (0..nodes.len())
        .find(|i| nodes.iter().all(|n| !n.contains(&(*i as i32))))
        .unwrap()
}

pub fn preorder_traversal(nodes: &Vec<Vec<i32>>, i: usize) {
    print!(" {}", i);
    if nodes[i][0] != -1 {
        preorder_traversal(nodes, nodes[i][0] as usize);
    }
    if nodes[i][1] != -1 {
        preorder_traversal(nodes, nodes[i][1] as usize);
    }
}

pub fn inorder_traversal(nodes: &Vec<Vec<i32>>, i: usize) {
    if nodes[i][0] != -1 {
        inorder_traversal(nodes, nodes[i][0] as usize);
    }
    print!(" {}", i);
    if nodes[i][1] != -1 {
        inorder_traversal(nodes, nodes[i][1] as usize);
    }
}

pub fn postorder_traversal(nodes: &Vec<Vec<i32>>, i: usize) {
    if nodes[i][0] != -1 {
        postorder_traversal(nodes, nodes[i][0] as usize);
    }
    if nodes[i][1] != -1 {
        postorder_traversal(nodes, nodes[i][1] as usize);
    }
    print!(" {}", i);
}

pub fn solve(nodes: Vec<Vec<i32>>) {
    let root = find_root(&nodes);

    println!("Preorder");
    preorder_traversal(&nodes, root);
    println!("");

    println!("Inorder");
    inorder_traversal(&nodes, root);
    println!("");

    println!("Postorder");
    postorder_traversal(&nodes, root);
    println!("");
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let mut nodes = iter
        .filter(|l| !l.is_empty())
        .map(|i| {
            let mut splitted = i.split(' ');
            let id = splitted.next().unwrap().parse::<usize>().unwrap();
            let children = splitted
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (id, children)
        })
        .collect::<Vec<(usize, Vec<i32>)>>();
    nodes.sort_by_key(|n| n.0);
    solve(nodes.into_iter().map(|n| n.1).collect());
}
