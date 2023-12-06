fn map_value(val: u64, range: &(u64, u64, u64)) -> u64
{   
    if val < range.1 || val > (range.1+range.2-1)
    {
        panic!("Value {val} is out of range d{} s{} l{}", range.0, range.1, range.2);
    }
    return val - range.1 + range.0;
}

// 79..92, 55..67
fn map_range(ranges: &Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)>
{
    let mut new_ranges: Vec<(u64, u64)> = vec![];
    for (start, end) in ranges{
        let mut s = *start;
        let e = *end;

        // Loop until the range is fully translated
        while true{
            let range_not_found = (s, e);
            // Find matching range
            let range = map.iter().find(|x|(s >= x.1) && (s<=(x.1+x.2 -1)));
            let range = match range {
                Some(x)=> x,
                _ => {
                    // If range is not found, search for a subset 
                    let range = map.iter().find(|x| (e >= x.1) && (e<=(x.1+x.2-1)));
                    match range {
                        // If new range is found, copy the subset and change start
                        Some(x) => {new_ranges.push((s,x.1-1));s = x.1;continue;},
                        //If range still isn't found, copy subset
                        _ => {new_ranges.push(range_not_found); break;}
                    };
                },
            };

            let range_end = range.1 + range.2 - 1;
            // If whole range translates
            if e <= range_end
            {
                let new_s = map_value(s, range);
                let new_e: u64 = map_value(e, range);
                new_ranges.push((new_s, new_e));
                break;
            }
            else if e > range_end {
                let new_s: u64 = map_value(s, range);
                let new_e: u64 = map_value(range_end, range);
                new_ranges.push((new_s, new_e));
                // Find the next range
                s = range_end + 1;
            }
        }
    }
    return new_ranges;
}

pub fn solve(lines: &Vec<String>) {
    let seeds  = &lines[0].split(" ").collect::<Vec<&str>>();
    let seeds_len = seeds.len();
    let seeds = seeds[1..seeds_len].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![];

    // Create and parse all maps
    let mut skip_next = false;
    for line in &lines[1..lines.len()]{
        if line == "" {
            maps.push(vec![]);
            skip_next = true;
            continue;
        }
        if skip_next{
            skip_next = false;
            continue;
        }
        let map = line.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let map_length = maps.len();
        maps[map_length - 1].push((map[0], map[1], map[2]));
    }

    let mut new_seeds: Vec<(u64, u64)> = vec![];

    for seed_idx in 0..seeds.len()/2{
        new_seeds.push((seeds[2*seed_idx], seeds[2*seed_idx]+seeds[2*seed_idx+1]-1));
    }

    for map in &maps{
        new_seeds = map_range(&new_seeds, &map);
    }

    println!("Part two result {}", new_seeds.iter().min_by_key(|x| x.0).unwrap().0);
}