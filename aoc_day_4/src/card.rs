
pub struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    pub id: u32,
    pub line: String,
    pub checked: bool,
}

impl Card {

    pub fn new(line: &str) -> Card {
        let line = line.replace("   ", " ");
        let line = line.replace("  ", " ");
        let temp = line.split(" ").collect::<Vec<&str>>();

        let mut card = Card {
            winning_numbers: vec![],
            numbers: vec![],
            id: temp[1].replace(":", "").parse::<u32>().unwrap(),
            line: line.clone(),
            checked: false,
        };

        let mut first_part: bool = true;

        for idx in 2..temp.len(){
            if "|" == temp[idx]{
                first_part = false;
                continue;
            }

            match temp[idx].parse::<u32>() {
                Ok(n) =>
                    if first_part{
                        card.winning_numbers.push(n)
                    }
                    else
                    {
                        card.numbers.push(n);
                    },
                Err(_err) => continue,
            }

        }
        return card;
    }

    pub fn value(&self) -> u32{
        let mut result = 0;
        for _ in 0..self.matches(){
            if 0 == result{
                result = 1;
            }
            else
            {
                result *= 2;
            }
        }
        return result;
    }

    pub fn matches(&self) -> u32{
        let mut result = 0;
        for w in &self.winning_numbers{
            for n in &self.numbers{
                if n == w {
                    result += 1;
                }
            }
        }
        return result;
    }
}