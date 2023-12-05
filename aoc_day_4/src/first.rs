#[path = "./card.rs"]
mod card;

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    for line in lines{
        let card = card::Card::new(line);
        res += card.value();
    }

    println!("Part one result {}", res);
}