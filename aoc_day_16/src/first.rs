use std::collections::HashMap;


#[derive(Eq, PartialEq, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}
const DOWN: u8 = 1;
const UP: u8 = 2;
const RIGHT: u8 = 4;
const LEFT: u8 = 8;

fn next(field: &Vec<Vec<char>>, position: &Point, prev_position: &Point) -> Vec<(Point, u8)> {
    let ch = field[position.y as usize][position.x as usize];
    // Going right
    if position.x - prev_position.x > 0 {
        return match ch {
            '|' => vec![(Point {x: position.x, y:position.y - 1}, UP), (Point {x: position.x, y:position.y + 1}, DOWN)],
            '-' => vec![(Point {x: position.x + 1, y:position.y}, RIGHT)],
            '/' => vec![(Point {x: position.x, y:position.y - 1}, UP)],
            '\\' => vec![(Point {x: position.x, y:position.y + 1}, DOWN)],
            '.' => vec![(Point {x: position.x + 1, y:position.y}, RIGHT)],
            x => panic!("{x}"),
        };
    }
    // Going left
    else if position.x - prev_position.x < 0 {
        return match ch {
            '|' => vec![(Point {x: position.x, y:position.y - 1}, UP), (Point {x: position.x, y:position.y + 1}, DOWN)],
            '-' => vec![(Point {x: position.x - 1, y:position.y}, LEFT)],
            '/' => vec![(Point {x: position.x, y:position.y + 1}, DOWN)],
            '\\' => vec![(Point {x: position.x, y:position.y - 1}, UP)],
            '.' => vec![(Point {x: position.x - 1, y:position.y}, LEFT)],
            x => panic!("{x}"),
        };
    }
    // Going down
    else if position.y - prev_position.y > 0 {
        return match ch {
            '|' => vec![(Point {x: position.x, y:position.y + 1}, DOWN)],
            '-' => vec![(Point {x: position.x - 1, y:position.y}, LEFT), (Point {x: position.x + 1, y:position.y},RIGHT)],
            '/' => vec![(Point {x: position.x - 1, y:position.y}, LEFT)],
            '\\' => vec![(Point {x: position.x + 1, y:position.y}, RIGHT)],
            '.' => vec![(Point {x: position.x, y:position.y + 1}, DOWN)],
            x => panic!("{x}"),
        };
    }
    // Going up
    else if position.y - prev_position.y < 0 {
        return match ch {
            '|' => vec![(Point {x: position.x, y:position.y - 1}, UP)],
            '-' => vec![(Point {x: position.x - 1, y:position.y}, LEFT), (Point {x: position.x + 1, y:position.y},RIGHT)],
            '/' => vec![(Point {x: position.x + 1, y:position.y}, RIGHT)],
            '\\' => vec![(Point {x: position.x - 1, y:position.y}, LEFT)],
            '.' => vec![(Point {x: position.x, y:position.y - 1}, UP)],
            x => panic!("{x}"),
        };
    }
    panic!("Not going anywhere.");
    
}
fn point_valid(point: &Point, xy_max: &Point) -> bool {
    return point.x >= 0 && point.y >= 0 && point.y < xy_max.y && point.x < xy_max.x;
}

fn get(field: &Vec<Vec<char>>, score: &mut Vec<Vec<u8>>, position: &Point, prev_position: &Point, deep: u32){

    // Get all next possible points
    let mut points = next(field, &position, &prev_position);
    let field_size = Point {x: field[0].len() as i32, y: field.len() as i32};

    // Mark the current position with all directions of these points, checking if the direction was already taken. If that happens, this node was already checked and the function can return.
    for (_, dir) in &points{
        if score[position.y as usize][position.x as usize] & dir > 0{
            return;
        }
        score[position.y as usize][position.x as usize] |= dir;
    }

    // Filter out all points that are outside the field.
    points = points.into_iter().filter(|(point, _dir)| point_valid(point, &field_size)).collect();

    // For every valid point, mark all spots untill the next mirror is found.    
    'outer: for (point, dir) in points{

        let mut prev_possible_point = position.clone();
        let mut possible_point = point;
        
        loop {
            score[prev_possible_point.y as usize][prev_possible_point.x as usize] |= dir;

            // If the path is leading outside the field, go process the next point.
            if !point_valid(&possible_point, &field_size){
                continue 'outer;
            }
            // If next point is a dot, check next point
            if field[possible_point.y as usize][possible_point.x as usize] == '.'{
                let (np, d) = next(field, &possible_point, &prev_possible_point)[0].clone();
                prev_possible_point = possible_point;
                possible_point = np;
                score[prev_possible_point.y as usize][prev_possible_point.x as usize] |= d;
            }
            else {
                break;
            }
        }

        // If a mirror is found, process it recursevly.
        get(field, score, &possible_point, &prev_possible_point, deep+1);
    }
}

pub fn solve(lines: &Vec<String>) {
    // Process input
    let field = lines.into_iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Prepare the score field
    let mut score: Vec<Vec<u8>> = vec![vec![0; field[0].len()]; field.len()];
    
    // Determine the starting points
    let starting_point = Point {x: 0, y: 0};
    let prev_point = Point{x: -1, y: 0};
    get(&field, &mut score, &starting_point, &prev_point, 0);

    let res: usize = score.iter().map(|row| row.into_iter().filter(|x| **x>0).count()).sum();

    println!("Part one result {}", res);
}