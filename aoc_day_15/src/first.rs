fn calculate_hash(code: &str) -> u32 {
    let mut hash: u32 = 0;
    for ch in code.to_ascii_lowercase().chars(){
        hash += ch as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let vals: Vec<&str> = lines[0].split(",").collect();
    for val in vals{
        let hash_val = calculate_hash(val);
        res += hash_val;
    }
    println!("Part one result {}", res);
}