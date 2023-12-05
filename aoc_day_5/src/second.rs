fn map_value(val: u64, dest: u64, src: u64, len: u64) -> u64
{
    return val - src + dest;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u64 = 0;

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


    let maps_len: usize = maps.len();
    maps[maps_len-1].sort_by_key(|x| x.0);
    let mut val = maps[0][0].0;
    for map in maps.iter().rev(){
        // Check if map translates this value and translate it
        for (dest, source, len) in map{
            if (val > *source) && val < (*source + *len)
            {
                let nval = map_value(val, *source, *dest, *len);
                val = nval;
                break;
            }
        }
    }

    println!("Part two result {}", res);
}