use std::io::Read;

pub fn posorder_traversal(preorder: &[usize], inorder: &[usize], result: &mut Vec<usize>) {
    let j = (0..inorder.len())
        .find(|j| preorder[0] == inorder[*j])
        .unwrap();
    if 0 < j {
        posorder_traversal(&preorder[1..=j], &inorder[..j], result);
    }
    if j + 1 < inorder.len() {
        posorder_traversal(&preorder[(j + 1)..], &inorder[(j + 1)..], result);
    }
    result.push(preorder[0]);
}

pub fn solve(preorder: Vec<usize>, inorder: Vec<usize>) {
    let mut result = Vec::new();
    posorder_traversal(&preorder, &inorder, &mut result);
    println!(
        "{}",
        result
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let orders = iter
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(' ')
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    solve(orders[0].clone(), orders[1].clone());
}
