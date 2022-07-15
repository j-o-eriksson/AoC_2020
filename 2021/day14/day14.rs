// AoC 2021 day 14

use std::collections::HashMap;

struct C20 {
    counts: HashMap<char, u64>,
    chain: Vec<char>,
}

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

fn count_elements(chain: &Vec<char>) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = HashMap::new();
    for c in chain {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        } else {
            counts.insert(*c, 1);
        }
    }
    counts
}

fn compute_c20(input_chain: &Vec<char>, rules: &HashMap<&str, char>) -> C20 {
    let mut chain: Vec<char> = input_chain.clone();
    for _ in 0..20 {
        let insertions = find_insertions(&chain, &rules);
        chain = next_chain(&chain, &insertions);
    }

    C20 {
        counts: count_elements(&chain),
        chain: chain,
    }
}

fn count_40(prev: &C20, rules: &HashMap<&str, C20>) -> C20 {
    let mut next = C20 {
        chain: Vec::new(),
        counts: "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .map(|c| (c, 0))
            .collect(),
    };
    prev.chain.windows(2).for_each(|w| {
        let pair = format!("{}{}", w[0], w[1]);
        let c20 = &rules.get(pair.as_str()).unwrap();
        for (key, value) in &c20.counts {
            *next.counts.get_mut(key).unwrap() += value;
        }
        *next.counts.get_mut(&w[1]).unwrap() -= 1;
    });

    next
}

fn main() {
    let (a, b) = include_str!("input").split_once("\n\n").unwrap();
    let rules: HashMap<&str, char> = b
        .lines()
        .map(|s| {
            let (pair, insert) = s.split_once(" -> ").unwrap();
            (pair, insert.chars().next().unwrap())
        })
        .collect();

    let c20s: HashMap<&str, C20> = rules
        .iter()
        .map(|(&s, _)| {
            println!("computing 20 for pair '{}'", s);
            let sv: Vec<char> = s.chars().collect();
            let c20 = compute_c20(&sv, &rules);
            (s, c20)
        })
        .collect();

    let mut counts: HashMap<char, u64> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .map(|c| (c, 0))
        .collect();

    let start_sequence: Vec<char> = a.chars().collect();
    start_sequence.windows(2).for_each(|s| {
        let pair = format!("{}{}", s[0], s[1]);
        println!("computing 40 for pair '{}'", pair);
        
        let pair20 = c20s.get(pair.as_str()).unwrap();
        let pair40 = count_40(pair20, &c20s);

        for (key, value) in &pair40.counts {
            *counts.get_mut(key).unwrap() += *value;
        }
    });
    *counts.get_mut(start_sequence.last().unwrap()).unwrap() += 1;

    let mut myvec: Vec<(char, u64)> = counts
        .into_iter()
        .filter(|(_, count)| *count != 0)
        .collect();
    myvec.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", myvec);
    println!("{:?}", myvec.first().unwrap().1 - myvec.last().unwrap().1);
}

// ABB

// (2) AB -> ABBCB
// AB -> ABBCB
// BB -> BCCAB
// BC -> BCCAC
// CB -> CBABB


// (4) AB
// 1: ABB
// 2: ABBCB
// 3: ABBCBCCAB
// 4: ABBC BCCA BCCA CBABB

// (2) BB -> BCCAB
// BC -> BCCAC
// CC -> C...B
// CA -> C...B
// AB -> A...B


// (4) BB
// 1: B.B
// 2: B...B
// 3: B.......B
// 4: B... .... .... ....B
