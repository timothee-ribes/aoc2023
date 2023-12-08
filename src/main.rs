use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod aoc4;
mod aoc5;

fn main() {
    if let Ok(lines) = read_lines("./inputs/aoc5.txt") {
        aoc5::aoc5(lines);
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
