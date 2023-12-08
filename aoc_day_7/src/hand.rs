
pub struct Hand {
    cards: Vec<char>,
    pub bid: u32,
    vals: Vec<char>,
}

fn check_five_of_a_kind(cards: &Vec<char>) -> Option<char>{
    if cards.iter().filter(|x| **x == cards[0]).count() == cards.len(){
        return Some(cards[0]);
    }
    return None;
}

fn check_four_of_a_kind(cards: &Vec<char>) -> Option<char>{
    for c in cards{
       if cards.iter().filter(|x| **x == *c).count() == 4
       {
            return Some(*c);
       }
    }
    return None;
}

fn check_fullhouse(cards: &Vec<char>) -> Option<char>{
    let mut cards_copy = cards.clone();
    let three = check_three_of_a_kind(&cards_copy);
    match three {
        Some(x) => {cards_copy = cards_copy.into_iter().filter(|c| *c != x).collect::<Vec<char>>(); return check_one_pair(&cards_copy); }
        None => return None
    }
}

fn check_three_of_a_kind(cards: &Vec<char>) -> Option<char>{
    for c in cards{
        if cards.iter().filter(|x| **x == *c).count() == 3
        {
                return Some(*c);
        }
    }
    return None;
}
fn check_two_pair(cards: &Vec<char>) -> Option<char>{
    let mut cards_copy = cards.clone();
    let onepair = check_one_pair(&cards_copy);
    match onepair {
        Some(x) => {cards_copy = cards_copy.into_iter().filter(|c| *c != x).collect::<Vec<char>>(); return check_one_pair(&cards_copy); }
        None => return None
    }
}

fn check_one_pair(cards: &Vec<char>) -> Option<char>{
    for c in cards{
        if cards.iter().filter(|x| **x == *c).count() == 2
        {
                return Some(*c);
        }
    }
    return None;
}

fn check_high_card(cards: &Vec<char>) -> bool{

    for c in cards{
        if cards.iter().filter(|x| **x == *c).count() > 1
        {
                return true;
        }
    }
    return false;
}



impl Hand {

    pub fn new(line: &str, importance: &Vec<char>) -> Hand {
        let temp = line.split(" ").collect::<Vec<&str>>();

        let hand: Hand = Hand {
            cards: temp[0].chars().collect(),
            bid: temp[1].parse::<u32>().unwrap(),
            vals: importance.clone(),
        };

        return hand;
    }

    pub fn score(&self) -> u32{
        if check_five_of_a_kind(&self.cards) != None{
            return 7;
        }
        if check_four_of_a_kind(&self.cards) != None{
            return 6;
        }
        if check_fullhouse(&self.cards) != None{
            return 5;
        }
        if check_three_of_a_kind(&self.cards) != None{
            return 4;
        }
        if check_two_pair(&self.cards) != None{
            return 3;
        }
        if check_one_pair(&self.cards) != None{
            return 2;
        }
        if check_high_card(&self.cards){
            return 1;
        }
        return 0;
    }

    pub fn print_score(&self){
        if check_five_of_a_kind(&self.cards) != None{
            print!(" five_of_a_kind ");
            return;
        }
        if check_four_of_a_kind(&self.cards) != None{
            print!(" four_of_a_kind ");
            return;
        }
        if check_fullhouse(&self.cards) != None{
            print!(" fullhouse ");
            return;
        }
        if check_three_of_a_kind(&self.cards) != None{
            print!(" three_of_a_kind ");
            return;
        }
        if check_two_pair(&self.cards) != None{
            print!(" two_pair ");
            return;
        }
        if check_one_pair(&self.cards) != None{
            print!(" one_pair ");
            return;
        }
        if check_high_card(&self.cards){
            print!(" high_card ");
            return;
        }
    }

    pub fn beats(&self, other: &Hand) -> bool {

        if self.score() == other.score() {
            for cidx in 0..self.cards.len(){
                if self.cards[cidx] != other.cards[cidx]{
                    // If smaller index, then it wins
                    return self.vals.iter().position(|x| *x == self.cards[cidx]).unwrap() < self.vals.iter().position(|x| *x == other.cards[cidx]).unwrap();
                }
            }
        }
        return  self.score() > other.score();
    }

    pub fn print(&self){
        for c in &self.cards{
            print!("{c}");
        }
        self.print_score();
        println!(" {} ", self.bid);
    }
}