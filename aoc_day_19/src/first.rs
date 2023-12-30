use std::collections::HashMap;

#[path = "./workflows.rs"]
mod workflows;
use workflows::Workflow;

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let mut workflows: Vec<Workflow> = vec![];
    let mut empty_line_detected = false;
    let mut vals: Vec<HashMap<char, u32>> = vec![];

    for line in lines{
        if line.is_empty(){
            empty_line_detected = true;
            continue;
        }
        if empty_line_detected.eq(&false){
            workflows.push(Workflow::new(line));
        }
        else {
            let parsed_line = line.replace("}", "");
            let ints: Vec<u32> = parsed_line.split(",").map(|x| x.split("=").collect::<Vec<&str>>()[1].parse::<u32>().unwrap()).collect();

            //let test: Vec<u32> = test.map(|x| x.split("=").collect::<Vec<&str>>()[0].parse::<u32>().unwrap()).collect();
            let hm: HashMap<char, u32> = HashMap::from([
                ('x', ints[0]),
                ('m', ints[1]),
                ('a', ints[2]),
                ('s', ints[3]),
            ]);
            vals.push(hm);
        }
    }
    for v in &vals{
        let mut wf = String::from("in");
        loop {
            let workflow = workflows.iter().find(|workflow| workflow.name.eq(&wf)).unwrap();
            wf = workflow.process(v);
            //println!("{wf} {v:?}");
            if wf.eq("A"){
                res += v.keys().map(|k| v.get(k).unwrap()).sum::<u32>();
                break;
            }
            if wf.eq("R"){
                break;
            }
        }
    }
    println!("Part one result {}", res);
}