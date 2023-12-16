use std::{collections::HashMap, vec};


#[derive(Eq, PartialEq, Clone)]
struct Point {
    pub x: usize,
    pub y: usize,
}

/* Get all possible points to go from one point. */
fn get_possible_points(map: &Vec<Vec<char>>, coordinate: &Point) -> Vec<Point> {
    return match map[coordinate.x][coordinate.y]{
        '|' => vec![Point {x: coordinate.x + 1, y: coordinate.y    }, Point {x: coordinate.x - 1, y: coordinate.y    }],
        '-' => vec![Point {x: coordinate.x    , y: coordinate.y + 1}, Point {x: coordinate.x    , y: coordinate.y - 1}],
        'L' => vec![Point {x: coordinate.x    , y: coordinate.y + 1}, Point {x: coordinate.x - 1, y: coordinate.y    }],
        'J' => vec![Point {x: coordinate.x    , y: coordinate.y - 1}, Point {x: coordinate.x - 1, y: coordinate.y    }],
        '7' => vec![Point {x: coordinate.x    , y: coordinate.y - 1}, Point {x: coordinate.x + 1, y: coordinate.y    }],
        'F' => vec![Point {x: coordinate.x    , y: coordinate.y + 1}, Point {x: coordinate.x + 1, y: coordinate.y    }],
        '.' => {panic!("Invalid value");},
        _ => panic!(""),
    }
}

// Determines the next position according to previous and current position
fn get_next_position(map: &Vec<Vec<char>>, previous_point: &Point, current_point: &Point) -> Point {
    let possible_points: Vec<Point> = get_possible_points(map, current_point);
    return possible_points.iter().filter(|x| *x != previous_point).collect::<Vec<&Point>>()[0].clone();
}

fn determine_start_point(map: &Vec<Vec<char>>, start_point: &Point) -> Point {
    let mut dirmap=  HashMap::<(i32, i32), Vec<char>>::new();
    dirmap.insert((1,0), vec!['L', '|', 'J']);
    dirmap.insert((-1,0), vec!['F', '|', '7']);
    dirmap.insert((0,1), vec!['J', '-', '7']);
    dirmap.insert((0,-1), vec!['L', '-', 'F']);

    for ((x,y), v) in dirmap{
        let new_x: i32 = start_point.x as i32 + x;
        let new_y: i32 = start_point.y as i32 + y;
        if new_x >= 0 && new_y >= 0 && new_x < map.len() as i32 && new_y < map[0].len() as i32{
            if v.contains(&map[new_x as usize][new_y as usize]){
                println!("Found starting point: {new_x}, {new_y}");
                return Point { x: new_x as usize, y: new_y as usize};
            }
        }
    }
    panic!("Starting point not found.");
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 1;
    let mut map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().into_iter().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Find Start point
    let mut start_point: Point= Point {x: 0, y:0};
    for x in 0..map.len(){
        let start = map[x].iter().position(|x| *x == 'S');
        if start != None{
            start_point = Point {x: x, y: start.unwrap()};
            break;
        }
    }

    let first_tile = determine_start_point(&map, &start_point);


    let mut current_point: Point = first_tile.clone();
    let mut previous_point: Point = start_point.clone();
    while current_point != start_point{
        map[previous_point.x][previous_point.y] = 'X';
        let new_point = get_next_position(&map, &previous_point, &current_point);
        previous_point = current_point;
        current_point = new_point;
    }
    map[previous_point.x][previous_point.y] = 'X';

    let mut start_end_total_points: Vec<(usize, usize)> = vec![];
    let map_len = map.len();
    for lidx in 0..map_len {
        let mut line: &mut Vec<char> = &mut map[lidx];
        let mut start_end_points: Vec<usize> = vec![];
        for (cidx, ch) in line.iter().enumerate(){
            let mut occurances = 0;
            if !ch.eq(&'X'){
                continue;
            }
            if cidx == 0 {
                occurances += 1
            }
            else if line.get(cidx - 1) != None {
                if *line.get(cidx - 1).unwrap() == 'X'{
                    occurances += 1;
                }
            }
            if line.get(cidx + 1) != None {
                if *line.get(cidx + 1).unwrap() == 'X'{
                    occurances += 1;
                }
            }
            if occurances < 2 {
                start_end_points.push(cidx);
            }
        }
        for pidx in 0..start_end_points.len()/2{
            let p1: usize = start_end_points[2*pidx];
            let p2: usize = start_end_points[2*pidx + 1];
            //if line[p1 + 1] != 'X'{
                start_end_total_points.push((p1, p2));
                for cidx in p1+1..p2{
                    line[cidx] = 'I';
                }
            //}
        }   
    }   
    
    for a in &map{
        for b in a{
            print!("{b}");
        }
        println!("");
    }
/*



    let mut found_total_points: Vec<(usize, usize)> = vec![];
    for (lidx, line) in map.iter().enumerate(){
        let mut start_y = 0;
        let mut found = false;
        let mut found_points: Vec<usize> = vec![];
        for cidx in 0..line.len(){
            // If first X was found.
            if found == false && line[cidx] == 'X'{
                found = true;
                start_y = cidx;
            }
            // If the end of X-es or end of line was found.
            else if found == true && line[cidx] != 'X'{
                found = false;
                if cidx - start_y == 1{
                    println!("Found point '{}' {}", lidx, start_y);
                    found_points.push(start_y);
                }
            }
        }
        // If found odd numer of points, remove the middle one
        if found_points.len() % 2 == 1{
            found_points.remove(found_points.len()/2);
        }
        for pidx in 0..found_points.len()/2{
            found_total_points.push((found_points[2*pidx], found_points[2*pidx+1]));
        }
    }*/
    let res = start_end_total_points.into_iter().map(|(x,y)| y-x - 1).collect::<Vec<usize>>();
    println!("Part two result {}", res.iter().sum::<usize>() );
}