use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let mut freq = 0;
    let mut seen = HashSet::new();
    let mut lines = Vec::new();

    let file = File::open("input").expect("Cannot read input");

    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        let line = line.parse::<i32>().expect("Unable to parse value");
        lines.push(line);
    }

    'outer: loop {
        for line in &lines {
            freq += line;
            if seen.contains(&freq) {
                break 'outer;
            }
            seen.insert(freq);
        }
    }

    println!("{}", freq);
}
