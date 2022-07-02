// AoC 2021 day 11

use std::fs;
use std::collections::HashSet;

const N: usize = 12;

struct Grid {
    data: [[u8; N]; N],
}

impl Grid {
    fn parse_grid(s: &str) -> Grid {
        let mut grid = Grid { data: [[0; N]; N] };

        for (i, row) in s.split("\n").enumerate() {
            for (j, val) in row.chars()
                .map(|c| c.to_digit(10).unwrap()).enumerate() {
                grid.data[i + 1][j + 1] = val as u8;
            }
        }

        grid
    }

    fn update(&mut self) {
        let mut flashes: HashSet<(usize, usize)> = HashSet::new();
        let mut primary_flashes: Vec<(usize, usize)> = Vec::new();
        let mut secondary_flashes: Vec<(usize, usize)> = Vec::new();

        for i in 1..(N-1) {
            for j in 1..(N-1) {
                self.data[i][j] += 1;
                if self.data[i][j] > 9 {
                    primary_flashes.push((i, j));
                    flashes.insert((i, j));
                }
            }
        }
        
        while true {
            for (i, j) in primary_flashes {
            }
            break;
        }
    }
}


const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1), 
    (-1,  0),          (1,  0), 
    (-1,  1), (0,  1), (1,  1), 
];

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read file.");
    let mut grid = Grid::parse_grid(&input);

    for i in 0..2 {
        grid.update();
    }

    for row in &grid.data {
        println!("{:?}", row);
    }
}
