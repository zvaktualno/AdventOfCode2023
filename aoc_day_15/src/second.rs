struct Lens{
    pub tag: String,
    pub f: u32,
}

struct Box{
    pub lenses: Vec<Lens>,
    pub id: u32,
}

impl Lens{
    pub fn new(tag: &String, f: u32) -> Lens {
        let lens = Lens {
            tag: tag.clone(),
            f: f
        };
        return lens;
    }
}

impl Box {
    pub fn new(id: u32) -> Box {
        let b = Box {
            id: id,
            lenses: vec![],
        };
        return b;
    }
    pub fn process_command(&mut self, cmd: &str) {
        if cmd.contains("-")
        {
            let tag = cmd.split("-");
            let tag = tag.collect::<Vec<&str>>();
            let tag = tag.first().unwrap();
            let tag = tag.to_string();

            let idx = self.lenses.iter().position(|x| x.tag.eq(&tag));
            if idx != None {
                //println!("Removing lens with tag {tag} from index {} at box {}", idx.unwrap(), self.id);
                self.lenses.remove(idx.unwrap());
            }
        }
        else if cmd.contains("=")
        {
            let split_string = cmd.split("=");
            let split_string = split_string.collect::<Vec<&str>>();
            let tag: &&str = split_string.first().unwrap();
            let tag = tag.to_string();
            let f = split_string[1].parse::<u32>().unwrap();

            let idx = self.lenses.iter().position(|x| x.tag.eq(&tag));
            if idx == None {
                let lens: Lens = Lens::new(&tag, f);
                self.lenses.push(lens);
                //println!("Adding a new lens with tag {tag} at box {}", self.id)
            }
            else{
                let lens: Lens = Lens::new(&tag, f);
                self.lenses[idx.unwrap()] = lens;
                //println!("Replacing a lens with tag {tag} at index {} at box {}", idx.unwrap(), self.id)
            }
        }
    }

    pub fn value(&self) -> u32 {
        let mut res = 0;

        for (lidx, lens) in self.lenses.iter().enumerate(){
            let mut lens_res = 1;
            lens_res *= self.id + 1;
            lens_res *= lidx as u32 + 1;
            lens_res *= lens.f;
            res += lens_res;
        }

        return res;
    }

    pub fn print(&self){
        print!("Box {}: ", self.id);
        for l in &self.lenses{
            print!("[{} {}] ", l.tag, l.f);
        }
        println!("");
    }
}

fn calculate_hash(code: &str) -> u32 {
    let mut hash: u32 = 0;
    for ch in code.to_ascii_lowercase().chars(){
        hash += ch as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u32 = 0;
    let vals: Vec<&str> = lines[0].split(",").collect();

    // Create boxes
    let mut boxes: Vec<Box> = vec![];
    for i in 0..256{
        boxes.push(Box::new(i));
    }

    // Process input
    for val in vals{
        let mut tag_len = val.len() - 1;
        if val.find("=") != None{
            tag_len -= 1;
        }
        let hash_val = calculate_hash(&val[0..tag_len]);
        boxes[hash_val as usize].process_command(val);

    }

    for b in &boxes{
        res += b.value();
    }

    println!("Part two result {}", res);
}