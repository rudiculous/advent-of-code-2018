use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Claim {
    id: i32,
    top: i32,
    left: i32,
    height: i32,
    width: i32,
}

impl Claim {
    fn parse(line: String) -> Claim {
        let (offset, id) = find_value(&line, "#", Some(" "), 0);
        let (offset, top) = find_value(&line, "@ ", Some(","), offset);
        let (offset, left) = find_value(&line, ",", Some(":"), offset);
        let (offset, height) = find_value(&line, ": ", Some("x"), offset);
        let (_, width) = find_value(&line, "x", None, offset);

        println!("{:?}", top);

        return Claim {
            id,
            top,
            left,
            height,
            width,
        };
    }
}

fn main() {
    let file = File::open("input").expect("Cannot read input");

    for line in BufReader::new(file).lines() {
        let line = line.expect("Unable to read line");
        println!("{:?}", Claim::parse(line));
    }
}

fn find(line: &String, from: usize, pat: &str) -> usize {
    return line[from..].find(pat).unwrap();
}

fn parse_value(line: &String, from: usize, to: Option<usize>) -> i32 {
    return match to {
        Some(to) => line[from..to].parse::<i32>().expect("Unable to parse value"),
        None => line[from..].parse::<i32>().expect("Unable to parse value"),
    };
}

fn find_value(line: &String, start: &str, end: Option<&str>, offset: usize) -> (usize, i32) {
    let from = find(&line, offset, start) + offset + start.len();
    return match end {
        Some(end) => {
            let to = find(&line, from, end) + from;
            return (to, parse_value(&line, from, Some(to)));
        }
        None => (line.len(), parse_value(&line, from, None)),
    };
}
