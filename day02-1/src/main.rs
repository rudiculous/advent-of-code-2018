use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let mut twos = 0;
    let mut threes = 0;

    let file = File::open("input").expect("Cannot read input");

    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");

        if check_freq(&line, 2) { twos += 1; }
        if check_freq(&line, 3) { threes += 1; }
    }

    println!("{}", twos * threes);
}

fn check_freq(line: &String, freq: i32) -> bool {
    let mut seen = HashMap::new();

    for c in line.chars() {
        if !seen.contains_key(&c) {
            seen.insert(c, 0);
        }

        let f = seen.get(&c).unwrap() + 1;
        seen.insert(c, f);
    }

    for v in seen.values() {
        if *v == freq {
            return true;
        }
    }

    return false;
}
