use std::collections::HashMap;

struct Point{
    pub x: i32,
    pub y: i32,
}


pub fn solve(lines: &Vec<String>) {
    let mut res: usize = 0;
    let dirs: HashMap<char, Point> = HashMap::from([
        ('U', Point {x: 0, y: -1}),
        ('D', Point {x: 0, y: 1}),
        ('R', Point {x: 1, y: 0}),
        ('L', Point {x: -1, y: 0}),
    ]);
    let mut field: Vec<Vec<char>> = vec![vec![' '; 1000]; 1000];
    let mut position: Point = Point {x:500, y:500};
    for line in lines{
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let ch: char = split_line[0].as_bytes()[0] as char;
        let number: u8 = split_line[1].parse::<u8>().unwrap();
        for _ in 0..number{
            field[position.y as usize][position.x as usize] = ch;
            position.x += dirs.get(&ch).unwrap().x;
            position.y += dirs.get(&ch).unwrap().y;
        }
    }
    for lidx in 0..field.len() - 1{
        let mut inside = false;
        for cidx in 0..field[0].len() {
            // detect a crossing
            if field[lidx][cidx] == 'D' || field[lidx+1][cidx] == 'U'{
                inside = !inside;
            }
            if field[lidx][cidx] == ' ' && inside{
                res += 1;
            }
        }
    }
    let outside_fields = field.iter().map(|line| line.iter().filter(|ch| **ch!=' ').count()).sum::<usize>();

    println!("Part one result {}", res + outside_fields);
}