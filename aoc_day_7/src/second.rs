use std::{cmp::Ordering, str::Chars, arch::global_asm, result, vec};
#[path = "./hand.rs"]
mod hand;

fn hcomp1(h1: &hand::Hand, h2:&hand::Hand) -> Ordering{

    return h1.beats(&h2).cmp(&true);
}

fn hcomp2(h1: &(hand::Hand, hand::Hand), h2:&(hand::Hand, hand::Hand)) -> Ordering{
    if h1.0.score() != h2.0.score(){
        return (h1.0.score() > h2.0.score()).cmp(&true);
    }
    let importance: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    for cidx in 0..h1.1.cards.len(){
        if h1.1.cards[cidx] != h2.1.cards[cidx]{
            // If smaller index, then it wins
            return (h1.1.vals.iter().position(|x| *x == h1.1.cards[cidx]).unwrap() < h1.1.vals.iter().position(|x| *x == h2.1.cards[cidx]).unwrap()).cmp(&true);
        }
    }
    return true.cmp(&true);
}
fn create_all_possible_cards(line: &String, start_idx: usize) -> Vec<hand::Hand>{
    let importance: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let card_len: usize = 5;
    let mut result: Vec<hand::Hand> = vec![];
    if start_idx == 5{
        return vec![hand::Hand::new(line, &importance)];
    }
    for idx in start_idx..card_len{
        if line.chars().collect::<Vec<char>>()[idx] == 'J'{
            for c in &importance{
                let mut new_line = line.clone();
                new_line.replace_range(idx..idx+1,  &c.to_string());
                let mut new_hands: Vec<hand::Hand> = create_all_possible_cards(&new_line, idx+1);
                result.append(&mut new_hands);
            }
            break;
        }
    }
    if result.len() == 0{
        return vec![hand::Hand::new(line, &importance)];
    }
    return result;
}

pub fn solve(lines: &Vec<String>) {

    let importance: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let mut hands: Vec<(hand::Hand, hand::Hand)> = vec![];
    for line in lines{
        let mut cards: Vec<hand::Hand> = create_all_possible_cards(line, 0);
        //println!("{}", cards.len());
        
        cards.sort_by(|hand1, hand2| hcomp1(hand2, hand1));
        //println!("Strongest card:");
        //cards[0].print();
        hands.push((cards[0].clone(), hand::Hand::new(line, &importance)));
    }

    hands.sort_by(|hand1, hand2| hcomp2(hand1, hand2));

    let mut res = 0;
    for (idx, hand) in hands.iter().enumerate(){
        res += ((idx as u32)+1) * hand.1.bid;
    }
    println!("Part two result {}", res);
}