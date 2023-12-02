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
    let filepath: &str = "in.txt";

    let lines = read_lines(filepath);
    first::solve(&lines);
    second::solve(&lines);
}