#[path = "./workflows.rs"]
mod workflows;
use workflows::Workflow;

fn recurse_workflows(wf: &String, wfs: &Vec<Workflow>, vals: &Vec<(u32, u32)>) -> u64{
    let mut res: u64 = 1;
    if wf.eq("A")
    {
        for v in vals{
            res *= v.1 as u64 - v.0 as u64 + 1;
        }

        println!("Checking {wf} with {vals:?}. Score {res}");
        return res;
    }
    else if wf.eq("R"){
        println!("Checking {wf} with {vals:?}");
        return 0;
    }
    println!("Checking {wf} with {vals:?}");
    let mut res = 0;

    let selected_wf = wfs.iter().find(|x| x.name.eq(wf)).unwrap();
    let mut x_limits = vals[0];
    let mut m_limits = vals[1];
    let mut a_limits = vals[2];
    let mut s_limits = vals[3];

    let x_cond = selected_wf.conditions.iter().find(|x| x.0.eq(&'x'));
    if x_cond != None{
        let (_ch, bigger, num, next_wf) = x_cond.unwrap();
        if *bigger {
            x_limits = (*num+1, vals[0].1);
            let new_vector: Vec<(u32, u32)> = vec![x_limits, vals[1], vals[2], vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
        else {
            x_limits = (vals[0].0, *num - 1);
            let new_vector: Vec<(u32, u32)> = vec![x_limits, vals[1], vals[2], vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
    }

    let m_cond = selected_wf.conditions.iter().find(|x| x.0.eq(&'m'));
    if m_cond != None{
        let (_ch, bigger, num, next_wf) = m_cond.unwrap();
        if *bigger {
            m_limits = (*num + 1, vals[1].1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], m_limits, vals[2], vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
        else {
            m_limits = (vals[1].0, *num - 1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], m_limits, vals[2], vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
    }

    let a_cond = selected_wf.conditions.iter().find(|x| x.0.eq(&'a'));
    if a_cond != None{
        let (_ch, bigger, num, next_wf) = a_cond.unwrap();
        if *bigger {
            a_limits = (*num + 1, vals[2].1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], vals[1], a_limits, vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
        else {
            a_limits = (vals[2].0, *num - 1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], vals[1], a_limits, vals[3]];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
    }

    let s_cond = selected_wf.conditions.iter().find(|x| x.0.eq(&'s'));
    if s_cond != None{
        let (_ch, bigger, num, next_wf) = s_cond.unwrap();
        if *bigger {
            s_limits = (*num + 1, vals[3].1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], vals[1], vals[2], s_limits];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
        else {
            s_limits = (vals[3].0, *num-1);
            let new_vector: Vec<(u32, u32)> = vec![vals[0], vals[1], vals[2], s_limits];
            res += recurse_workflows(next_wf, wfs, &new_vector);
        }
    }

    if vals[0].0.eq(&x_limits.0) && !vals[0].1.eq(&x_limits.1){
        x_limits = (x_limits.1 + 1, vals[0].1);
    }
    else if !vals[0].0.eq(&x_limits.0) && vals[0].1.eq(&x_limits.1){
        x_limits = (vals[0].0, x_limits.0 - 1);
    }

    if vals[1].0.eq(&m_limits.0) && !vals[1].1.eq(&m_limits.1){
        m_limits = (m_limits.1 + 1, vals[1].1);
    }
    else if !vals[1].0.eq(&m_limits.0) && vals[1].1.eq(&m_limits.1){
        m_limits = (vals[1].0, m_limits.0 - 1);
    }

    if vals[2].0.eq(&a_limits.0) && !vals[2].1.eq(&a_limits.1){
        a_limits = (a_limits.1 + 1, vals[2].1);
    }
    else if !vals[2].0.eq(&a_limits.0) && vals[2].1.eq(&a_limits.1){
        a_limits = (vals[2].0, a_limits.0 - 1);
    }

    if vals[3].0.eq(&s_limits.0) && !vals[3].1.eq(&s_limits.1){
        s_limits = (s_limits.1 + 1, vals[3].1);
    }
    else if !vals[3].0.eq(&s_limits.0) && vals[3].1.eq(&s_limits.1){
        s_limits = (vals[3].0, s_limits.0 - 1);
    }


    res += recurse_workflows(&selected_wf.default, wfs, &vec![x_limits, m_limits, a_limits, s_limits]);
    return res;
}

pub fn solve(lines: &Vec<String>) {
    let mut res: u64 = 0;
    let mut workflows: Vec<Workflow> = vec![];
    let mut empty_line_detected = false;

    for line in lines{
        if line.is_empty(){
            empty_line_detected = true;
            continue;
        }
        if empty_line_detected.eq(&false){
            workflows.push(Workflow::new(line));
        }
    }

    let start_intervals: Vec<(u32, u32)> = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    res = recurse_workflows(&String::from("in"), &workflows, &start_intervals);

    println!("Part two result {}", res);
}