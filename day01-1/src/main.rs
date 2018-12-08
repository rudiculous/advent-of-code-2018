use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut freq = 0;

    let file = File::open("input").expect("Cannot read input");

    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        let line = line.parse::<i32>().expect("Unable to parse value");
        freq += line;
    }

    println!("{}", freq);
}
