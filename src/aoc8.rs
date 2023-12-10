use std::collections::HashMap;

use crate::file::read_lines;

pub fn run() {
    if let Ok(mut lines) = read_lines("inputs/8-full.txt") {
        let first_line = lines.next().unwrap().unwrap();
        println!("{}", first_line);
        lines.next();
        let mut network = HashMap::new();
        let mut starts = vec![];
        loop {
            if let Some(Ok(node)) = lines.next() {
                println!("{}", node);
                let source = String::from(&node[0..3]);
                let left = String::from(&node[7..10]);
                let right = String::from(&node[12..15]);
                println!("{}:{}:{}", source, left, right);
                network.insert(source.clone(), (left, right));
                if source.chars().nth(2).unwrap() == 'A' {
                    starts.push(source);
                }
            } else {
                break;
            }
        }
        println!("{:?}", network);
        println!("{:?}", starts);
        let mut ends = vec![];
        let mut stepss = vec![];
        let mut cycle_len: Vec<(u64, u64)> = vec![];
        for s in starts {
            let mut steps = 0;
            let mut i = 0;
            println!("start:{}", s);
            let mut current_node: String = s.clone();
            while !ends.contains(&current_node) {
                if current_node.chars().nth(2).unwrap() == 'Z' {
                    ends.push(current_node.clone());
                    stepss.push(steps);
                }
                if first_line.chars().nth(i).unwrap() == 'R' {
                    current_node = network.get(&current_node).unwrap().1.clone();
                } else {
                    current_node = network.get(&current_node).unwrap().0.clone();
                }
                steps += 1;
                i = (i + 1) % first_line.len();
            }
            cycle_len.push((stepss[stepss.len() - 1], steps - stepss[stepss.len() - 1]));
        }
        println!("end_dist{:?}", stepss);
        println!("end{:?}", ends);
        println!("cycle{:?}", cycle_len);

        let mut farest_node = cycle_len
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .clone();
        loop {
            // println!("{:?}", farest_node);
            if cycle_len.iter().all(|x| (farest_node.0 - x.0) % x.1 == 0) {
                break;
            }
            farest_node = (farest_node.0 + farest_node.1, farest_node.1);
        }
        println!("{:?}", farest_node);
    } else {
        println!("Error: No file")
    }
}
