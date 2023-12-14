use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    DOT,
    SQUARE,
    ROUND
}

pub fn solve(lines: &Vec<String>) {
    let mut res: usize = 0;
    let mut field: Vec<Vec<TileType>> = vec![];

    let mut vals: HashMap<char, TileType> = HashMap::new();
    vals.insert('.', TileType::DOT);
    vals.insert('O', TileType::ROUND);
    vals.insert('#', TileType::SQUARE);

    for line in lines{
        let test: Vec<TileType> =line.chars().map(|val| *vals.get(&val).unwrap()).collect::<Vec<TileType>>();
        field.push(test);
    }

    let mut moved = true;
    while moved {
        moved = false;
        for lidx in 0..field.len(){
            for cidx in 0..field[lidx].len(){
                if field[lidx][cidx] == TileType::ROUND && !lidx.eq(&0) && field[lidx - 1][cidx] == TileType::DOT{
                    field[lidx - 1][cidx] = field[lidx][cidx];
                    field[lidx][cidx] = TileType::DOT;
                    moved = true
                }
            }
        }
    }
    for lidx in 0..field.len(){
        for cidx in 0..field[lidx].len(){
            if field[lidx][cidx] == TileType::ROUND{
                res += field.len() - lidx;
            }
        }
    }
    println!("Part one result {}", res);
}