use std::{collections::HashMap, cmp::{min, max}};
fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn solve(lines: &Vec<String>) {
    let dirs: Vec<char> = lines[0].chars().collect();

    let mut field_map = HashMap::<String, (String, String)>::new();

    // Parse map
    for line in lines[2..lines.len()].iter(){
        let cleaned_str = line.replace("=", "").replace("(", "").replace(")", "").replace(",", "");
        let parsed_str: Vec<String> = cleaned_str.split_whitespace().map(|x| String::from(x)).collect();
        field_map.insert(parsed_str[0].clone(), (parsed_str[1].clone(), parsed_str[2].clone()));
    }

    // Select starting values
    let mut starting_fields = field_map.keys().filter(|x| x.chars().collect::<Vec<char>>()[2]=='A');
    let mut starting_fields = starting_fields.collect::<Vec<&String>>().into_iter().map(|x| x.clone()).collect::<Vec<String>>();

    let mut steps: usize = 0;
    let mut field_solutions: Vec<u64> = vec![0; starting_fields.len()];

    // Compute all starting values separatly
    while field_solutions.iter().any(|x| x.eq(&0)) {
        for fidx in 0..starting_fields.len(){
            if field_solutions[fidx] == 0
            {
                let (left, right) = field_map.get(&starting_fields[fidx]).unwrap();

                if dirs[steps % dirs.len()] == 'L'{
                    starting_fields[fidx] = left.clone();
                }
                else{
                    starting_fields[fidx] = right.clone();
                }

                // If search ended, save steps
                if starting_fields[fidx].chars().collect::<Vec<char>>()[2]=='Z'
                {
                    field_solutions[fidx]= (steps as u64) + 1;
                }
            }
        }
        steps += 1;
    }

    // The result is the LCM of all numbers
    let mut res = 1;
    for fs in field_solutions
    {
        res = lcm(res, fs);
    }
    println!("Part two result {}", res);
}