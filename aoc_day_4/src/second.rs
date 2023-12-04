#[path = "./card.rs"]
mod card;

pub fn solve(lines: &Vec<String>) {
    let mut cards: Vec<card::Card> = vec![];

    for line in lines{
        let card = card::Card::new(line);
        cards.push(card);
    }

    while true{
        let mut new_cards: Vec<card::Card> = vec![];

        for card in &cards{
            if card.checked == true{
                continue;
            }
            let card_matches = card.matches();
            for idx in 0..card_matches
            {
                let card_idx: u32 = card.id + idx;
                let new_card: &card::Card = &cards[card_idx as usize];
                let new_card = card::Card::new(new_card.line.as_str());
                new_cards.push(new_card);
            }
        }

        if new_cards.len() == 0{
            break;
        }

        for card in &mut cards{
            card.checked = true;
        }

        cards.append(&mut new_cards);
    }


    println!("Part two result {}", cards.len());
}