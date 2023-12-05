use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::path::Path;

mod aoc4;

fn main() {
    if let Ok(lines) = read_lines("./inputs/aoc5.txt") {
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
                    let mut new_seeds = vec![];
                    while seeds.len() > 0 {
                        let mut new_ranges = vec![];
                        'a: for s in &seeds {
                            for v in &range_map {
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
    } else {
        println!("Error: Failed to open");
    }
}

//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
