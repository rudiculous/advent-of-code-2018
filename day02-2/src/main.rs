use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = Vec::new();

    let file = File::open("input").expect("Cannot read input");

    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        lines.push(line);
    }

    for line1 in &lines {
        for line2 in &lines {
            if distance(line1, line2) == 1 {
                println!("{}, {}, {}", line1, line2, diff(line1, line2));
            }
        }
    }
}

fn distance(line1: &String, line2: &String) -> i32 {
    let mut dist = 0;
    let mut line2 = line2.chars();

    for c1 in line1.chars() {
        let c2 = line2.next().unwrap(); // assumption: all box ids have equal length

        if c1 != c2 { dist += 1; }
    }

    return dist;
}

fn diff(line1: &String, line2: &String) -> String {
    let mut dff = "".to_owned();
    let mut line2 = line2.chars();

    for c1 in line1.chars() {
        let c2 = line2.next().unwrap(); // assumption: all box ids have equal length

        if c1 == c2 {
            dff.push(c1);
        }
    }

    return dff;
}
