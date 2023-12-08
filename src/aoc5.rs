use std::{
    fs::File,
    io::{self, BufReader, Lines},
    ops::ControlFlow,
};

fn parse_range_map(lines_it: &mut Lines<BufReader<File>>) -> Vec<(i64, i64, i64)> {
    let mut range_map = vec![];
    loop {
        if let Some(Ok(l)) = lines_it.next() {
            let range: Vec<i64> = l
                .clone()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            if range.len() == 0 {
                break;
            }
            range_map.push((range[0], range[1], range[2]));
        } else {
            break;
        }
    }
    println!("{:?},{}", range_map, range_map.len());
    range_map
}
pub fn aoc5(lines: io::Lines<io::BufReader<File>>) {
    let mut lines_it = lines.into_iter();
    let seeds: Vec<String> = match lines_it.next() {
        Some(Ok(ip)) => ip.clone().split(":").map(|x| String::from(x)).collect(),
        _ => return,
    };
    let mut seeds_range = seeds[1]
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());
    let mut seeds = vec![];
    loop {
        if let Some(v1) = seeds_range.next() {
            println!("{}", v1);
            if let Some(v2) = seeds_range.next() {
                println!("{}", v2);
                seeds.push((v1, v2));
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("{:?}", seeds);
    loop {
        if let Some(Ok(ip)) = lines_it.next() {
            if ip.len() > 0 && &ip[ip.len() - 1..ip.len()] == ":" {
                println!("{}", ip);
                let range_map = parse_range_map(&mut lines_it);
                let mut new_seeds = vec![];
                while seeds.len() > 0 {
                    let mut new_ranges = vec![];
                    'a: for s in &seeds {
                        for v in &range_map {
                            if let ControlFlow::Break(_) =
                                push_if_intersect_and_split(s, v, &mut new_ranges, &mut new_seeds)
                            {
                                continue 'a;
                            }
                        }
                        new_seeds.push(*s);
                    }
                    seeds = new_ranges.clone();
                }
                println!("{:?}, {:?}", new_seeds, new_seeds.iter().min());
                seeds = new_seeds;
            }
        } else {
            break;
        }
    }
}

fn push_if_intersect_and_split(
    s: &(i64, i64),
    v: &(i64, i64, i64),
    new_ranges: &mut Vec<(i64, i64)>,
    new_seeds: &mut Vec<(i64, i64)>,
) -> ControlFlow<()> {
    let (l, r) = s;
    let r = l + r;
    if r > v.1 && *l < v.2 + v.1 {
        let new_l = l.max(&v.1);
        let new_r = r.min(v.1 + v.2) - new_l;

        if new_l > &s.0 {
            new_ranges.push((s.0, new_l - s.0));
        }
        if new_l + new_r < &s.0 + &s.1 {
            new_ranges.push((new_l + new_r, s.0 + s.1 - new_l - new_r));
        }

        let new_l = new_l - v.1 + v.0;

        new_seeds.push((new_l, new_r));
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
