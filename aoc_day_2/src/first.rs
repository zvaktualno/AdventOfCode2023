use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;

    // Using multiple values to avoid parsing ',', ';'...
    let colors = HashMap::from([
        ("blue", 0),
        ("red", 1),
        ("green", 2),
        ("blue,", 0),
        ("red,", 1),
        ("green,", 2),
        ("blue;", 0),
        ("red;", 1),
        ("green;", 2)
    ]);

    let max_colors: Vec<u32> = vec![14,12,13];

    // Iterate over games
    for line in lines{
        // Split line into colors
        let mut color_bins: Vec<u32> = vec![0,0,0];
        let split_line: Vec<&str> = line.split(" ").collect();

        // Check each color
        for x in split_line.iter().enumerate()
        {
            if colors.contains_key(x.1){
                let bin_num = colors.get(x.1).unwrap();
                let occurances = String::from(split_line[x.0 - 1]).parse::<u32>().unwrap();
                // Use the max value
                color_bins[*bin_num] = std::cmp::max(color_bins[*bin_num], occurances);
            }
        }
        if max_colors.iter().zip(color_bins.iter()).filter(|&(a, b)| a >= b).count() == max_colors.len()
        {
            // Parse and add game number
            let game_num = split_line[1][..split_line[1].len()-1].parse::<i32>().unwrap();
            res += game_num;
        }
    }
    println!("Part one result {}", res);
}