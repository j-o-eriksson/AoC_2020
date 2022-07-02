// AoC 2021 day 10

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum ParsingError {
    Syntax(char),
    Incomplete(Vec<char>),
}

fn parse_chunk(chunk: &str) -> ParsingError {
    let m1: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let m2: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut stack: Vec<char> = Vec::new();

    for c in chunk.chars() {
        if m1.contains_key(&c) {
            stack.push(c);
        } else if m2.contains_key(&c) {
            let last = stack.pop().unwrap();
            if last != m2[&c] {
                return ParsingError::Syntax(c);
            }
        } else {
            println!("{}", "error!");
        }
    }

    ParsingError::Incomplete(stack)
}

fn syntax_error_score(c: &ParsingError) -> usize {
    match c {
        ParsingError::Incomplete(_) => 0,
        ParsingError::Syntax(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
    }
}

fn incomplete_error_score(c: &ParsingError) -> usize {
    match c {
        ParsingError::Syntax(_) => 0,
        ParsingError::Incomplete(stack) => {
            let score: usize = stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                })
                .fold(0, |acc, val| acc * 5 + val);
            score
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file.");
    let errors: Vec<ParsingError> = contents.split("\n").map(parse_chunk).collect();

    // part 1
    println!("{:?}", errors.iter().map(syntax_error_score).sum::<usize>());

    // part 2
    let mut v = errors
        .iter()
        .map(incomplete_error_score)
        .filter(|&x| x > 0)
        .collect::<Vec<usize>>();
    v.sort();
    println!("{:?}", v[v.len() / 2]);
}
