use std::f32::DIGITS;

fn has_one_difference(s1: &Vec<char>, s2: &Vec<char>) -> bool {
    let mut found_one_difference = false;

    for (c1, c2) in s1.iter().zip(s2.iter()) {
        if c1 != c2 {
            if found_one_difference {
                return false;
            } else {
                found_one_difference = true
            }
        }
    }

    found_one_difference
}

fn check_pattern(pattern: &Vec<&String>) -> usize {
    let mut res = 0;
    println!("Checking rows");

    'rows: for lidx in 1..pattern.len(){
        let mut first_idx = lidx - 1;
        let mut second_idx = lidx;

        // Find the first row that repeats
        if !pattern[first_idx].eq(pattern[second_idx])
        {
            continue;
        }

        // Find exactly one smudge
        let mut diffs_found = 0;
        loop {
            if first_idx == 0 || second_idx == pattern.len() - 1{
                println!("Diffs found {diffs_found}");
                if diffs_found == 1{
                    break;
                }
                println!("Diffs found {diffs_found}");
                break 'rows;
            }
            first_idx -= 1;
            second_idx += 1;

            let row1: Vec<char> = pattern[first_idx].chars().collect();
            let row2: Vec<char> = pattern[second_idx].chars().collect();
            if has_one_difference(&row1, &row2){
                println!("Found one diff at line {first_idx}");
                diffs_found += 1;
            }
        }
        println!("lidx: {lidx}");
        return 100 * lidx;
    }

    'cols: for cidx in 1..pattern[0].len(){
        let mut first_idx = cidx - 1;
        let mut second_idx: usize = cidx;
        let mut col1: Vec<char> = pattern.iter().map(|x| x.chars().nth(first_idx).unwrap()).collect();
        let mut col2: Vec<char> = pattern.iter().map(|x| x.chars().nth(second_idx).unwrap()).collect();

        // Find the first col that repeats
        if !col1.eq(&col2)
        {
            continue;
        }

        // Find exactly one smudge
        let mut diffs_found = 0;
        loop {
            if first_idx == 0 || second_idx == pattern[0].len() - 1{
                println!("Diffs found {diffs_found}");
                if diffs_found == 1{
                    break;
                }
                println!("Diffs found {diffs_found}");
                break 'cols;
            }
            first_idx -= 1;
            second_idx += 1;

            let row1: Vec<char> = pattern.iter().map(|x| x.chars().nth(first_idx).unwrap()).collect();
            let row2: Vec<char> = pattern.iter().map(|x| x.chars().nth(second_idx).unwrap()).collect();
            if has_one_difference(&row1, &row2){
                println!("Found one diff at line {first_idx}");
                diffs_found += 1;
            }
        }
        return cidx;
    }
    return res;
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
    println!("Part two result {}", res);
}