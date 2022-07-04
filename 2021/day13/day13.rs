// AoC 2021 day 13

use std::fs;

const N: usize = 15;

struct Grid {
    data: [[u8; N]; N],
}

impl Grid {
    fn add_point(&mut self, p: (usize, usize)) {
        self.data[p.0][p.1] += 1;
    }

    fn fold_along_x(&self, x: usize) -> Grid {
        let mut grid = Grid {
            data: [[0; N]; N],
        };
        grid
    }
}

fn from_string(s: &str) -> (usize, usize) {
    let v: Vec<&str> = s.split(",").collect();
    (v[0].parse().unwrap(), v[1].parse().unwrap())
}

fn main() {
    let input = fs::read_to_string("test_input").expect("Failed to read file.");
    let a: Vec<&str> = input.split("\n\n").collect();
    let points: Vec<(usize, usize)> = a[0].split("\n").map(from_string).collect();

    let mut grid = Grid {
        data: [[0; N]; N],
    };

    for p in points {
        grid.add_point(p);
    }

    println!("{:?}", grid.data);
}
