const EXPANSION: i64 = 2;

fn calculate_distance_p2p(universe: &Vec<Vec<bool>>, c1: &(i64, i64), c2: &(i64, i64)) -> i64{
    let mut x_step: i64 = 0;
    if (c1.0 - c2.0) != 0 {
        x_step = (c2.0 - c1.0)/(c1.0 - c2.0).abs();
    }
    let mut y_step: i64 = 0;
    if (c1.1 - c2.1) != 0 {
        y_step = (c2.1 - c1.1)/(c1.1 - c2.1).abs();
    }
    let mut distance = 0;
    let mut current_x = c1.0;
    let mut current_y = c1.1;

    // Go over verticals, checking if any line is empty
    while current_x != c2.0{
        if universe[current_x as usize].iter().all(|x| *x==false){
            distance += EXPANSION;
        }
        else
        {
            distance += 1;
        }
        current_x += x_step;
    }

    // Go over horizontals, checking if any column is empty
    while current_y != c2.1{
        if universe.iter().all(|x| x[current_y as usize]==false){
            distance += EXPANSION;
        }
        else
        {
            distance += 1;
        }
        current_y += y_step;
    }
    return distance;

}

pub fn solve(lines: &Vec<String>) {
    let mut res: i64 = 0;
    let mut universe: Vec<Vec<bool>> = vec![];

    // Parse universe
    for line in lines{
        let parsed_line: Vec<bool> = line.chars().map(|x| x=='#').collect();
        universe.push(parsed_line);
    }

    let mut coordinates: Vec<(i64, i64)> = vec![];

    // Save all coordinates
    for (lidx, line) in universe.iter().enumerate(){
        for (cidx, ch) in line.iter().enumerate(){
            if ch.eq(&true){
                coordinates.push((lidx as i64, cidx as i64));
            }
        }
    }

    for x in 0..(coordinates.len()-1){
        for y in (x+1)..coordinates.len(){
            res += calculate_distance_p2p(&universe, &coordinates[x], &coordinates[y]);
        }
    }
    println!("Part one result {}", res);
}