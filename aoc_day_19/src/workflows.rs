use std::collections::HashMap;

pub struct Workflow {
    pub name: String,
    pub conditions: Vec<(char, bool, u32, String)>,
    pub default: String
}

impl Workflow {
    pub fn new(line: &String) -> Workflow {
        let ws_start = line.find('{').unwrap() + 1;
        let ws_end = line.find('}').unwrap();

        let string_conditions = line[ws_start..ws_end].split(",").collect::<Vec<&str>>();

        let mut conditions: Vec<(char, bool, u32, String)> = vec![];
        // Loop over all but last
        for c in 0..string_conditions.len()-1{
            if string_conditions[c].find("<") != None{
                let tmp: Vec<&str> = string_conditions[c].split("<").collect();
                let var: char = tmp[0].as_bytes()[0] as char;
                let tmp: Vec<&str> = tmp[1].split(":").collect();
                let num: u32 = tmp[0].parse::<u32>().unwrap();
                let name: &str = tmp[1];
                conditions.push((var, false, num, name.to_string()));
            }
            else
            {
                let tmp: Vec<&str> = string_conditions[c].split(">").collect();
                let var: char = tmp[0].as_bytes()[0] as char;
                let tmp: Vec<&str> = tmp[1].split(":").collect();
                let num: u32 = tmp[0].parse::<u32>().unwrap();
                let name: &str = tmp[1];
                conditions.push((var, true, num, name.to_string()));
            }
        }
        let wf: Workflow = Workflow { name: line[0..ws_start-1].to_string(), conditions: conditions, default: string_conditions.last().unwrap().to_string() };
        println!("Creating a workflow '{}' ", wf.name);
        return wf;
    }

    pub fn process(&self, val: &HashMap<char, u32>) -> String{
        for (ch, bigger, v, res) in &self.conditions{
            if *bigger {
                if val.get(ch).unwrap() > v {
                    return res.clone();
                }
            }
            else {
                if val.get(ch).unwrap() < v {
                    return res.clone();
                }
            }
        }
        return self.default.clone();
    }
}