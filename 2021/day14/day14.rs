// AoC 2021 day 13

use std::collections::HashMap;

fn find_insertions(chain: &Vec<char>, rules: &HashMap<&str, char>) -> Vec<(char, usize)> {
    let mut insertions: Vec<(char, usize)> = Vec::new();
    chain.windows(2).enumerate().for_each(|(i, w)| {
        let pair = format!("{}{}", w[0], w[1]);
        match rules.get(pair.as_str()) {
            Some(c) => insertions.push((*c, i)),
            None => (),
        }
    });
    insertions
}

fn main() {
    let (a, b) = include_str!("test_input").split_once("\n\n").unwrap();
    let rules: HashMap<&str, char> = b
        .lines()
        .map(|s| {
            let (pair, insert) = s.split_once(" -> ").unwrap();
            (pair, insert.chars().next().unwrap())
        })
        .collect();
    println!("{:?}", rules);

    let chain: Vec<char> = a.chars().collect();

    for i in 0..(chain.len() - 1) {
        let pair: String = format!("{}{}", chain[i], chain[i + 1]);
        match rules.get(pair.as_str()) {
            Some(c) => println!("insert {} after {}", c, i),
            None => (),
        }
    }

    println!("{:?}", find_insertions(&chain, &rules));
}
