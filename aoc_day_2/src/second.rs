use std::collections::HashMap;

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
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

                color_bins[*bin_num] = std::cmp::max(color_bins[*bin_num], occurances);
            }
        }
        res += color_bins[0] * color_bins[1] * color_bins[2];

    }
    println!("Part two result {}", res);
}