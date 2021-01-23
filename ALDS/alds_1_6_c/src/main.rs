use std::io::Read;

pub fn partition(cards: &mut Vec<(&str, i32, usize)>, p: usize, r: usize) -> usize {
    let x = cards[r].1;
    let mut i = p;
    for j in p..r {
        if cards[j].1 <= x {
            i += 1;
            cards.swap(i - 1, j)
        }
    }
    cards.swap(i, r);
    i
}

pub fn quicksort(cards: &mut Vec<(&str, i32, usize)>, p: usize, r: usize) {
    if p < r {
        let q = partition(cards, p, r);
        quicksort(cards, p, q - 1);
        quicksort(cards, q + 1, r);
    }
}

pub fn check_stable(nums: &Vec<(&str, i32, usize)>) -> String {
    let mut i = 0;
    while i < nums.len() {
        let mut j = i;
        while j < nums.len() - 1 && nums[i].1 == nums[j + 1].1 {
            j += 1;
            if nums[j - 1].2 > nums[j].2 {
                return "Not stable".to_string();
            }
        }
        i = j + 1;
    }
    "Stable".to_string()
}

pub fn solve(mut cards: Vec<(&str, i32, usize)>) {
    let l = cards.len();
    quicksort(&mut cards, 0, l - 1);

    println!("{}", check_stable(&cards));
    for c in cards {
        println!("{} {}", c.0, c.1);
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n');
    let _n = iter.next().unwrap();
    let cards = iter
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(i, l)| {
            let mut splitted = l.split(' ');
            (
                splitted.next().unwrap(),
                splitted.next().unwrap().parse::<i32>().unwrap(),
                i,
            )
        })
        .collect();
    solve(cards);
}
