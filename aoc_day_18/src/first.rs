use std::collections::HashMap;

struct Point{
    pub x: i32,
    pub y: i32,
}


pub fn solve(lines: &Vec<String>) {

    let dirs: HashMap<char, Point> = HashMap::from([
        ('U', Point {x: 0, y: -1}),
        ('D', Point {x: 0, y: 1}),
        ('R', Point {x: 1, y: 0}),
        ('L', Point {x: -1, y: 0}),
    ]);
    let mut field: Vec<Vec<char>> = vec![vec![' '; 7]; 10];
    let mut position: Point = Point {x:0, y:0};
    for line in lines{
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let ch: char = split_line[0].as_bytes()[0] as char;
        let number: u8 = split_line[1].parse::<u8>().unwrap();
        for _ in 0..number{
            position.x += dirs.get(&ch).unwrap().x;
            position.y += dirs.get(&ch).unwrap().y;
            field[position.y as usize][position.x as usize] = 'O';
        }
    }
    for a in field{for b in a{print!("{b}");}println!("");}
    println!("Part one result {}", 0);
}