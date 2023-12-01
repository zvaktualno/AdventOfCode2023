pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    for s in lines
    {
        let mut digits: Vec<char> = vec![];
        for c in s.chars()
        {
            if c >= '0' && c <= '9'
            {
                digits.push(c);
            }
        }
        res += digits[0].to_digit(10).unwrap()*10 + digits.last().unwrap().to_digit(10).unwrap();
    }
    println!("Part one result {}", res);
}