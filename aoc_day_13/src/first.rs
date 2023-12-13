fn check_pattern(pattern: &Vec<&String>) -> usize {

    for lidx in 1..pattern.len(){
        let mut first_idx = lidx - 1;
        let mut second_idx = lidx;
        while pattern[first_idx].eq(pattern[second_idx])
        {
            println!("Found matching rows {first_idx} and {second_idx}");
            if first_idx == 0 || second_idx == pattern.len() - 1
            {
                println!("{pattern:?}");
                return 100 * lidx;
            }
            first_idx -= 1;
            second_idx += 1;
        }
    }

    for cidx in 1..pattern[0].len(){
        let mut first_idx = cidx - 1;
        let mut second_idx = cidx;
        let mut col1: Vec<char> = pattern.iter().map(|x| x.chars().nth(first_idx).unwrap()).collect();
        let mut col2: Vec<char> = pattern.iter().map(|x| x.chars().nth(second_idx).unwrap()).collect();
        while col1.eq(&col2)
        {
            println!("Found matching cols {first_idx} and {second_idx}");
            if first_idx == 0 || second_idx == pattern[0].len() - 1
            {
                return cidx;
            }
            first_idx -= 1;
            second_idx += 1;
            col1 = pattern.iter().map(|x| x.chars().nth(first_idx).unwrap()).collect();
            col2 = pattern.iter().map(|x| x.chars().nth(second_idx).unwrap()).collect();
        }
    }
    panic!("Not found.");
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let mut pattern: Vec<&String> = vec![];
    for line in lines{
        if line.is_empty(){
            res += check_pattern(&pattern) as u32;
            pattern = vec![];
        }
        else {
            pattern.push(line);
        }
    }
    res += check_pattern(&pattern) as u32;
    println!("Part one result {}", res);
}