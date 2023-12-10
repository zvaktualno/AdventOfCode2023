use std::{collections::HashMap, vec};


#[derive(Eq, PartialEq, Clone)]
struct Point {
    pub x: usize,
    pub y: usize,
}

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
fn get_next_position(coordinate: &Point, map: &Vec<Vec<char>>, prev_coordinate: &Point) -> Point {
    let possible_points = get_possible_points(map, coordinate);
    for possible_point in possible_points{
        if possible_point != *prev_coordinate{
            return  possible_point;
        }
    }
    panic!("Can't find next position");
}

fn determine_start_points(start_point: &Point, map: &Vec<Vec<char>>) -> Vec<Point> {
    let mut dirmap=  HashMap::<(i32, i32), Vec<char>>::new();
    dirmap.insert((1,0), vec!['L', '|', 'J']);
    dirmap.insert((-1,0), vec!['F', '|', '7']);
    dirmap.insert((0,1), vec!['J', '-', '7']);
    dirmap.insert((0,-1), vec!['L', '-', 'F']);
    let mut res: Vec<Point> = vec![];
    for ((x,y), v) in dirmap{
        let new_x: i32 = start_point.x as i32 + x;
        let new_y: i32 = start_point.y as i32 + y;
        if new_x >= 0 && new_y >= 0 && new_x < map.len() as i32 && new_y < map[0].len() as i32{
            if v.contains(&map[new_x as usize][new_y as usize]){
                println!("Found starting point: {new_x}, {new_y}");
                res.push(Point { x: new_x as usize, y: new_y as usize});
            }
        }
    }
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 1;
    let mut map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().into_iter().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut s_point: Point= Point {x: 0, y:0};
    for x in 0..map.len(){
        let start = map[x].iter().position(|x| *x == 'S');
        if start != None{
            s_point = Point {x: x, y: start.unwrap()};
            break;
        }
    }

    let starting_points = determine_start_points(&s_point, &map);


    let mut p1: Point = starting_points[0].clone();
    let mut p2: Point = starting_points[1].clone();
    let mut prev_p1: Point = s_point.clone();
    let mut prev_p2: Point = s_point.clone();
    map[s_point.x][s_point.y] = 'X';
    while p1 != p2{
        res += 1;
        let new_p1 = get_next_position(&p1, &map, &prev_p1);
        let new_p2 = get_next_position(&p2, &map, &prev_p2);
        map[p1.x][p1.y] = 'X';
        map[p2.x][p2.y] = 'X';
        prev_p1 = p1;
        prev_p2 = p2;
        p1 = new_p1;
        p2 = new_p2;
    }
    map[p1.x][p1.y] = 'X';

    for a in &map{
        for b in a{
            print!("{b}");
        }
        println!("");
    }

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
    }
    let res = found_total_points.into_iter().map(|(x,y)| y-x - 1).collect::<Vec<usize>>();
    println!("Part two result {}", res.iter().sum::<usize>() );
}