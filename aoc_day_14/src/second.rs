use std::collections::HashMap;

#[allow(dead_code)]
enum Direction{
    NORTH = 1,
    WEST = 2,
    SOUTH,
    EAST
}

fn tilt_field(round_rocks: &mut Vec<(usize, usize)>, sq_rocks: &Vec<(usize, usize)>, dir: &Direction, field_size: (usize, usize)){
    let mut changed = true;
    while changed {
        changed = false;
        match dir {
            Direction::SOUTH => {
                for ridx in 0..round_rocks.len(){
                    let mut new_rock = round_rocks[ridx];
                    if new_rock.1 != field_size.1 - 1{
                        new_rock.1 += 1;
                        if !sq_rocks.contains(&new_rock) && !round_rocks.contains(&new_rock) {
                                round_rocks[ridx] = new_rock;
                                changed = true;
                        }
                    }
                }
            },
            Direction::WEST => {
                for ridx in 0..round_rocks.len(){
                    let mut new_rock = round_rocks[ridx];
                    if new_rock.0 != 0{
                        new_rock.0 -= 1;
                        if !sq_rocks.contains(&new_rock) && !round_rocks.contains(&new_rock){
                            round_rocks[ridx] = new_rock;
                            changed = true;
                        }
                    }
                }
            },
            Direction::NORTH => {
                for ridx in 0..round_rocks.len(){
                    let mut new_rock = round_rocks[ridx];
                    if new_rock.1 != 0{
                        new_rock.1 -= 1;
                        if !sq_rocks.contains(&new_rock) && !round_rocks.contains(&new_rock){
                            round_rocks[ridx] = new_rock;
                            changed = true;
                        }
                    }
                }
            },
            Direction::EAST => {
                for ridx in 0..round_rocks.len(){
                    let mut new_rock = round_rocks[ridx];
                    if new_rock.0 != field_size.0 - 1{
                        new_rock.0 += 1;
                        if !sq_rocks.contains(&new_rock) && !round_rocks.contains(&new_rock){
                            round_rocks[ridx] = new_rock;
                            changed = true;
                        }
                    }
                }
            },
        }
    }
}

pub fn solve(lines: &Vec<String>) {
    let mut res: usize = 0;

    let mut round_rocks: Vec<(usize, usize)> = vec![];
    let mut square_rocks: Vec<(usize, usize)> = vec![];
    for (lidx, line) in lines.iter().enumerate(){
        for (cidx, ch) in line.chars().enumerate(){
            if ch == '#'{
                square_rocks.push((cidx, lidx));
            }
            else if ch == 'O' {
                round_rocks.push((cidx, lidx));
            }
        }
    }

    println!("Found {} square rocks and {} round rocks", square_rocks.len(), round_rocks.len());
    let field_dimensions = (lines[0].len(), lines.len());

    let mut last_eight: HashMap<usize, usize> = HashMap::new();
    for i in 0..150 {
        tilt_field(&mut round_rocks, &square_rocks, &Direction::NORTH, field_dimensions);
        tilt_field(&mut round_rocks, &square_rocks, &Direction::WEST, field_dimensions);
        tilt_field(&mut round_rocks, &square_rocks, &Direction::SOUTH, field_dimensions);
        tilt_field(&mut round_rocks, &square_rocks, &Direction::EAST, field_dimensions);
        let mut res: usize = 0;
        for round_rock in &round_rocks{
            res += field_dimensions.1 - round_rock.1;
        }
        println!("{i} {res}");
    }


    println!("The result is the repeting pattern");
}