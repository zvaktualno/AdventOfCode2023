fn solve_time_distance(time: u64, distance: u64) -> u64
{
    let mut result = 0;

    for t in 0..time {
        let speed = t * 1;
        if distance <= speed * (time - t) {
            result += 1;
        }
    }
    return  result;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u64 = 0;
    let time = lines[0].replace("Time:", "").replace(" ", "").parse::<u64>().unwrap();
    let distance = lines[1].replace("Distance:", "").replace(" ", "").parse::<u64>().unwrap();

    res = solve_time_distance(time, distance);

    println!("Part two result {}", res);
}