use std::fs::read_to_string;
mod first;
mod second;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn main() {
    let filepath: &str = "test2.txt";
    use std::time::Instant;

    let lines = read_lines(filepath);

    let now = Instant::now();
    //first::solve(&lines);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    let now = Instant::now();
    second::solve(&lines);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
}