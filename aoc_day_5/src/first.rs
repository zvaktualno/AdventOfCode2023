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

    let mut locations: Vec<u64> = vec![];
    // Go over seeds
    for seed in seeds{
        let mut val:u64 = seed;

        // Go over all maps
        for map in &maps{

            // Check if map translates this value and translate it
            for (dest, source, len) in map{
                if (val > *source) && val < (*source + *len)
                {
                    let nval = map_value(val, *dest, *source, *len);
                    val = nval;
                    break;
                }
            }
        }
        locations.push(val);
    }
    res = *locations.iter().min().unwrap();
    println!("Part one result {}", res);
}