use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;

    let word_digits =  HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for s in lines
    {
        let mut digits: Vec<char> = vec![];
        let mut wstring_buffer: String = String::from("");
        for c in s.chars()
        {
            wstring_buffer.push(c);
            // Convert char to int
            if c >= '0' && c <= '9'
            {
                digits.push(c);
            }
            else
            {
                // Check if any of word digits in string buffer
                for wdigit in word_digits.keys()
                {
                    if wstring_buffer.contains(wdigit)
                    {
                        let ch: char = *word_digits.get(wdigit).unwrap();
                        digits.push(ch);
                        wstring_buffer = String::new();
                        wstring_buffer.push_str(wdigit);
                        wstring_buffer.remove(0);
                    }
                }
            }
        }
        let line_result = digits[0].to_digit(10).unwrap()*10 + digits.last().unwrap().to_digit(10).unwrap();
        res += line_result;
    }
    println!("Part two result {}", res);
}