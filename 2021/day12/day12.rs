// AoC 2021 day 11

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Location {
    Start,
    End,
    Small(String),
    Large(String),
}

fn to_location(s: &str) -> Location {
    match s {
        "start" => Location::Start,
        "end" => Location::End,
        l if l.chars().all(|c| c.is_lowercase()) => Location::Small(l.to_string()),
        u => Location::Large(u.to_string()),
    }
}

#[derive(Debug, Clone)]
struct Connection {
    a: Location,
    b: Location,
}

impl Connection {
    fn from_string(s: &str) -> Connection {
        let ss: Vec<&str> = s.split("-").collect();
        Connection {
            a: to_location(ss[0]),
            b: to_location(ss[1]),
        }
    }
}

fn add_connection(
    steps: &mut HashMap<Location, HashSet<Location>>,
    loc_a: Location,
    loc_b: Location,
) {
    if let Some(locs) = steps.get_mut(&loc_a) {
        locs.insert(loc_b);
    } else {
        steps.insert(loc_a, HashSet::from([loc_b]));
    }
}

fn next_path(path: &Vec<Location>, loc: &Location) -> Vec<Location> {
    let mut next_path = path.clone();
    next_path.push(loc.clone());
    next_path
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read file.");
    let connections: Vec<Connection> = input
        .split("\n")
        .filter(|&s| s.len() > 0)
        .map(Connection::from_string)
        .collect();

    let mut steps = HashMap::new();
    for c in &connections {
        add_connection(&mut steps, c.a.clone(), c.b.clone());
        add_connection(&mut steps, c.b.clone(), c.a.clone());
    }

    let mut count: u32 = 0;
    let mut paths = vec![vec![Location::Start]];
    while !paths.is_empty() {
        let prev_paths = paths.clone();
        paths.clear();
        for path in &prev_paths {
            let tail = path.last().unwrap();
            if let Some(next_steps) = steps.get(tail) {
                for step in next_steps {
                    match step {
                        Location::Start => {}
                        Location::End => {
                            count += 1;
                        }
                        Location::Small(_) => {
                            if let None = path.iter().find(|&x| x == step) {
                                paths.push(next_path(path, step));
                            }
                        }
                        Location::Large(_) => {
                            paths.push(next_path(path, step));
                        }
                    }
                }
            }
        }
        println!("{}", paths.len());
    }
    println!("Part 1: {:?} possible paths", count);
}
