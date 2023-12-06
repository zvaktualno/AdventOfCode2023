fn solve_time_distance(time: u64, distance: u64) -> u64
{
    let mut result = 0;

    for t in 0..time {
        let speed = t * 1;
        if distance < speed * (time - t) {
            result += 1;
        }
    }
    return  result;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u64 = 1;

    let times = lines[0].split_whitespace().collect::<Vec<&str>>();
    let times = times[1..times.len()].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let distances = lines[1].split_whitespace().collect::<Vec<&str>>();
    let distances = distances[1..distances.len()].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    for i in 0..times.len(){
        res *= solve_time_distance(times[i], distances[i]);
    }
    println!("Part one result {}", res);
}