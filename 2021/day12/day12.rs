// AoC 2021 day 11

use std::fs;

#[derive(Debug)]
enum Location {
    Small(String),
    Large(String),
    Start,
    End,
}

#[derive(Debug)]
struct Path {
    a: Location,
    b: Location,
}

fn to_location(s: &str) -> Location {
    match s {
        "end" => Location::End,
        "start" => Location::Start,
        s if s.chars().all(|c| c.is_lowercase()) => Location::Small(s.to_string()),
        s => Location::Large(s.to_string()),
    }
}

impl Path {
    fn from_string(s: &str) -> Path {
        let ss: Vec<&str> = s.split("-").collect();
        Path {
            a: to_location(ss[0]),
            b: to_location(ss[1]),
        }
    }
}

fn main() {
    let input = fs::read_to_string("test_input").expect("Failed to read file.");
    let paths: Vec<Path> = input
        .split("\n")
        .filter(|&s| s.len() > 0)
        .map(Path::from_string)
        .collect();

    for p in paths {
        println!("{:?}-{:?}", p.a, p.b);
    }
}
