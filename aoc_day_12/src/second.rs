use std::vec;

// ???.### 1,1,3
// .??..??...?##. 1,1,3
fn is_special_char(ch:char) -> bool{
    return ch == '#' || ch == '?';
}

fn get_bit_combinations(bit: &str) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = vec![];
    if bit.chars().any(|x| !x.eq(&'#'))
    {
        let mut combinations: Vec<String> = vec![bit.to_string()];
        //println!("Starting combination {combinations:?}");
        while combinations.iter().any(|x| x.chars().any(|c| c.eq(&'?'))) {
            let mut new_combinations: Vec<String> = vec![];

            for combination in &combinations{
                if combination.find('?') != None {
                    new_combinations.push(combination.replacen('?', &"#", 1));
                    new_combinations.push(combination.replacen('?', &".", 1));
                }
                else
                {
                    new_combinations.push(combination.clone());
                }
            }
            combinations = new_combinations.clone();
        }
        for combination in &combinations
        {  
            let test = combination.replace(".", " ");
            res.push(test.split_whitespace().map(|x| x.len() as u32).collect::<Vec<u32>>())
        }
        //println!("found combinations {res:?}");
    }
    else
    {
        return vec![vec![bit.len() as u32]];
    }

    return res;
}

fn number_of_combinations(field: &String, nums: &Vec<u32>) -> u32{
    let mut res: u32 = 0;
    let parsed_field = field.replace(".", " ");
    let bits = parsed_field.split_whitespace().collect::<Vec<&str>>();
    println!("{field} {nums:?}");
    for bit in &bits{
        let combinations = get_bit_combinations(bit);
        println!("{combinations:?}");
        let bit_res = combinations.into_iter().filter(|x| x.eq(nums)).count() as u32;

        println!("{bit} {bit_res}");
        res += bit_res;
    }
    println!("");
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let field = split_line[0];
        //let field: String = String::from(field).repeat(5);
        let nums: Vec<u32> = split_line[1].split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        //let nums = nums.repeat(5);
        res += number_of_combinations(&field.to_string(), &nums);
    }
    println!("Part two result {}", res);
}