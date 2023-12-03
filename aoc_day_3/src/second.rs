fn is_number(c: char) -> bool {
    if c >= '0' && c <= '9'{
        return true;
    }
    return false;
}

fn is_symbol(c: char) -> bool {
    if c == '*'{
        return true;
    }
    return false;
}

fn check_if_part_number(map: &Vec<String>, gears: &mut Vec<Vec<Vec<u32>>>, line_idx: usize, start: usize, end: usize) -> u32 {
    let e = &map[line_idx][start..end];
    let e = e.parse::<u32>().unwrap();

    let start_check = if start > 0 {start - 1} else {start};
    let end_check = if end < map[line_idx].len() {end + 1} else {end};

    for i in start_check..end_check{
        if line_idx > 0{
            if is_symbol(map[line_idx-1].chars().collect::<Vec<char>>()[i]){
                gears[line_idx-1][i].push(e);
                return e;
            }
        }
        if line_idx < (map.len() - 1){
            if is_symbol(map[line_idx+1].chars().collect::<Vec<char>>()[i]){
                gears[line_idx+1][i].push(e);
                return e;
            }
        }

    }
    if start > 0 {
        if is_symbol(map[line_idx].chars().collect::<Vec<char>>()[start-1]){
            gears[line_idx][start-1].push(e);
            return e;
        }
    }
    if end < map[0].len()
    {
        if is_symbol(map[line_idx].chars().collect::<Vec<char>>()[end]){
            gears[line_idx][end].push(e);
            return e;
        }
    }
    return 0;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    // Define an invalid value to compare to
    let inv_val: usize = 10000;

    // Create a map with same dimensions to enter gears.
    let mut gears:Vec<Vec<Vec<u32>>>= vec![vec![vec![]; lines[0].len()]; lines.len()];

    // Iterate over all chars
    for (line_idx, line) in lines.iter().enumerate(){
        let mut start: usize = inv_val;
        for (char_idx, ch) in line.chars().enumerate(){

            // If a new number is detected
            if is_number(ch) && start == inv_val{
                start = char_idx;
            }

            // If number previously detected
            else if start != inv_val{
                // Check if non num symbol detected
                if !is_number(ch){
                    check_if_part_number(lines, &mut gears, line_idx, start, char_idx);
                    start = inv_val;
                }
                // Check if the string ended
                else if char_idx == (lines[line_idx].len()-1)
                {
                    check_if_part_number(lines, &mut gears, line_idx, start, char_idx+1);
                    start = inv_val;

                }
            }
        }
    }

    // Sum the multiplicated gears if there is more than one entry per gear.
    for lgears in gears{
        for gear in lgears{
            if gear.len() > 1{
                let mut mult = 1;
                for g in gear{
                    mult *= g;
                }
                res += mult;
            }
        }
    }
    println!("Part two result {}", res);
}