fn check_string_if_ok(field: &String, nums: &Vec<u32>) -> u32 {
    let mut start_idx: usize = 0;

    for num in nums {
        // Check start_idx
        if start_idx > field.len(){
            return 0;
        }
        match field[start_idx..field.len()].find('#')
        {
            Some(x) => {start_idx = start_idx + x;},
            None => {return 0;}
        }
        let end_idx: usize = start_idx + *num as usize;
        
        // Check end_idx
        if end_idx > field.len(){
            return 0;
        }

        if field[start_idx..end_idx].chars().all(|f| f=='#')
        {
            if end_idx < field.len() && !field[end_idx..end_idx+1].contains('.'){
                return 0;
            }
            start_idx = end_idx + 1;
        }
        else
        {
            return 0;    
        }
    }
    if start_idx > field.len(){
        return 1;
    }
    if field[start_idx..field.len()].find('#') != None{
        return 0;
    }
    return 1;
}

fn number_of_combinations(field: &String, nums: &Vec<u32>) -> u32{
    let mut field_variations = vec![field.clone()];
    let mut res: u32 = 0;
    // Make all string with possible combinations of '?'
    loop {
        if !field_variations.iter().any(|x| x.chars().any(|x| x == '?')) {
            break;
        }
        let fv_len = field_variations.len();

        for fidx in (0..fv_len).rev(){
            if field_variations[fidx].find('?') != None
            {
                let str1 = field_variations[fidx].replacen('?', &"#", 1);
                field_variations[fidx] = field_variations[fidx].replacen('?', &".", 1);

                if str1.find('?') == None{
                    res += check_string_if_ok(&str1, nums);
                }
                else
                {
                    field_variations.push(str1);
                }
                if field_variations[fidx].find('?') == None{
                    res += check_string_if_ok(&field_variations[fidx], nums);
                    field_variations.remove(fidx);
                }
            }
        }
    }
    
    
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let field = split_line[0];
        let nums: Vec<u32> = split_line[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        
        res += number_of_combinations(&field.to_string(), &nums);
    }
    println!("Part one result {}", res);
}