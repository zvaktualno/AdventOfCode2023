use std::cmp::Ordering;
#[path = "./hand.rs"]
mod hand;

fn hcomp(h1: &hand::Hand, h2: &hand::Hand) -> Ordering{
    return h1.beats(h2).cmp(&true);
}

pub fn solve(lines: &Vec<String>) {
    let importance: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let mut hands = lines.iter().map(|line| hand::Hand::new(line, &importance)).collect::<Vec<hand::Hand>>();
    hands.sort_by(|hand1, hand2| hcomp(hand1, hand2));

    let mut res: u32 = 0;
    for (idx, hand) in hands.iter().enumerate(){
        res += ((idx as u32)+1) * hand.bid;
    }
    println!("Part one result {}", res);
}