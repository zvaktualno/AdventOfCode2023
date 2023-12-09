fn get_diff_vec(input: &Vec<i32>) -> Vec<i32>{
    return input.windows(2).map(|x| x[1]-x[0]).collect::<Vec<i32>>();
}

pub fn solve(lines: &Vec<String>) {
    let mut res: i32 = 0;
    for line in lines{
        let parsed_line: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut layers: Vec<Vec<i32>> = vec![parsed_line];

        while !layers.last().unwrap().iter().all(|x| *x == 0) {

            layers.push(get_diff_vec(&layers.last().unwrap()));
        }

        let num_of_layers = layers.len();
        for lidx in (1..num_of_layers).rev(){
            let new_val = -layers[lidx].first().unwrap() + layers[lidx - 1].first().unwrap();
            layers[lidx-1].insert(0,new_val);
        }

        res += layers[0].first().unwrap();
    }
    println!("Part two result {}", res);
}