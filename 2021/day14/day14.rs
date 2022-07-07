// AoC 2021 day 13

use std::collections::HashMap;
use std::collections::HashSet;

fn find_insertions(chain: &Vec<char>, rules: &HashMap<&str, char>) -> Vec<(char, usize)> {
    let mut insertions: Vec<(char, usize)> = Vec::new();
    chain.windows(2).enumerate().for_each(|(i, w)| {
        let pair = format!("{}{}", w[0], w[1]);
        match rules.get(pair.as_str()) {
            Some(c) => insertions.push((*c, i + 1)),
            None => (),
        }
    });
    insertions
}

fn next_chain(chain: &Vec<char>, insertions: &Vec<(char, usize)>) -> Vec<char> {
    let mut next_chain = Vec::new();

    let mut i0 = 0;
    for (c, i1) in insertions {
        for i in i0..*i1 {
            next_chain.push(chain[i]);
        }
        next_chain.push(*c);
        i0 = *i1;
    }

    for i in i0..chain.len() {
        next_chain.push(chain[i]);
    }

    next_chain
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

    let mut chain: Vec<char> = a.chars().collect();

    for _ in 0..10 {
        let insertions = find_insertions(&chain, &rules);
        chain = next_chain(&chain, &insertions);
    }

    let set: HashSet<&char> = chain.iter().collect();
    for c in &set {
        let count = chain.iter().filter(|ci| ci == c).count();
        println!("{}: {}", c, count);
    }
    println!("{:?}", chain.len());
    println!("{:?}", set);
}
