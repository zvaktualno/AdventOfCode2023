use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) {
    let dirs: Vec<char> = lines[0].chars().collect();

    let mut field_map = HashMap::<String, (String, String)>::new();

    // Parse map
    for line in lines[2..lines.len()].iter(){
        let cleaned_str = line.replace("=", "").replace("(", "").replace(")", "").replace(",", "");
        let parsed_str: Vec<String> = cleaned_str.split_whitespace().map(|x| String::from(x)).collect();
        field_map.insert(parsed_str[0].clone(), (parsed_str[1].clone(), parsed_str[2].clone()));
    }

    // Solve for "AAA"
    let mut field = &String::from("AAA");
    let mut steps: usize = 0;
    while !field.eq("ZZZ") {
        if dirs[steps % dirs.len()] == 'L'{
            field = &field_map.get(field).unwrap().0;
        }
        else{
            field = &field_map.get(field).unwrap().1;
        }
        steps += 1;
    }
    println!("Part one result {}", steps);
}