#[allow(dead_code)]
enum Direction{
    NORTH = 1,
    WEST = 2,
    SOUTH,
    EAST
}

fn tilt_field(round_rocks: &mut Vec<(usize, usize)>, sq_rocks: &Vec<(usize, usize)>, dir: &Direction, field_size: (usize, usize)){

    match dir {
        Direction::SOUTH => {
            for ridx in 0..round_rocks.len(){
                let mut new_rock = round_rocks[ridx];
                if new_rock.1 != 0{
                    new_rock.1 -= 1;
                    if !round_rocks.contains(&new_rock) && !sq_rocks.contains(&new_rock){
                        round_rocks[ridx] = new_rock;
                    }
                }
            }
        },
        Direction::WEST => {
            for ridx in 0..round_rocks.len(){
                let mut new_rock = round_rocks[ridx];
                if new_rock.0 != 0{
                    new_rock.0 -= 1;
                    if !round_rocks.contains(&new_rock) && !sq_rocks.contains(&new_rock){
                        round_rocks[ridx] = new_rock;
                    }
                }
            }
        },
        Direction::NORTH => {
            for ridx in 0..round_rocks.len(){
                let mut new_rock = round_rocks[ridx];
                if new_rock.1 != field_size.1 - 1{
                    new_rock.1 += 1;
                    if !round_rocks.contains(&new_rock) && !sq_rocks.contains(&new_rock){
                        round_rocks[ridx] = new_rock;
                    }
                }
            }
        },
        Direction::EAST => {
            for ridx in 0..round_rocks.len(){
                let mut new_rock = round_rocks[ridx];
                if new_rock.0 != field_size.0 - 1{
                    new_rock.0 += 1;
                    if !round_rocks.contains(&new_rock) && !sq_rocks.contains(&new_rock){
                        round_rocks[ridx] = new_rock;
                    }
                }
            }
        },
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
    let field_dimensions = (lines[0].len(), lines.len());

    for x in 0..1 {
        if x % 100000 == 0 {println!("{x}");}
        tilt_field(&mut round_rocks, &square_rocks, &Direction::NORTH, field_dimensions);
        //tilt_field(&mut round_rocks, &square_rocks, &Direction::WEST, field_dimensions);
        //tilt_field(&mut round_rocks, &square_rocks, &Direction::SOUTH, field_dimensions);
        //tilt_field(&mut round_rocks, &square_rocks, &Direction::EAST, field_dimensions);
    }

    for round_rock in round_rocks{
        res += field_dimensions.1 - round_rock.1;
        println!("{round_rock:?}");
    }

    println!("Part one result {}", res);
}