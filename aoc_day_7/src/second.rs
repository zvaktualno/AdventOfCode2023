use std::{cmp::Ordering, str::Chars, arch::global_asm};
#[path = "./hand.rs"]
mod hand;

fn hcomp(h1: &hand::Hand, h2: &hand::Hand) -> Ordering{
    return h1.beats(h2).cmp(&true);
}

pub fn solve(lines: &Vec<String>) {

    let importance: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let mut hands: Vec<hand::Hand> = vec![];
    for line in lines{
        let mut joker_positions: Vec<usize> = vec![];
        for idx in 0..line.len(){
            if line.as_bytes()[idx] == 'C' as u8 {
                joker_positions.push(idx);
            }
        }
    }

    hands.sort_by(|hand1, hand2| hcomp(hand1, hand2));

    let mut res: u32 = 0;
    for (idx, hand) in hands.iter().enumerate(){
        res += ((idx as u32)+1) * hand.bid;
    }
    println!("Part two result {}", res);
}