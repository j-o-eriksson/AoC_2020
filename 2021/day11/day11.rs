// AoC 2021 day 11

use std::fs;

const N: usize = 12;

struct Grid {
    data: [[u8; N]; N],
    flash_count: u32,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Grid {
    fn parse_grid(s: &str) -> Grid {
        let mut grid = Grid { data: [[0; N]; N], flash_count: 0 };

        for (i, row) in s.split("\n").enumerate() {
            for (j, val) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                grid.data[i + 1][j + 1] = val as u8;
            }
        }

        grid
    }

    fn increment(&mut self, i: usize, j: usize) -> bool {
        if (i == 0 || i == N - 1) || (j == 0 || j == N - 1) {
            return false;
        }
        self.data[i][j] += 1;
        return self.data[i][j] == 10;
    }

    fn update(&mut self) {
        let mut secondary_flashes: Vec<(usize, usize)> = Vec::new();

        // step 1
        for i in 1..(N - 1) {
            for j in 1..(N - 1) {
                if self.increment(i, j) {
                    secondary_flashes.push((i, j));
                }
            }
        }

        // step 2
        while secondary_flashes.len() > 0 {
            let (i, j) = secondary_flashes.pop().unwrap();
            for (di, dj) in &NEIGHBORS {
                let ii = (i as i32 + di) as usize;
                let jj = (j as i32 + dj) as usize;
                if self.increment(ii, jj) {
                    secondary_flashes.push((ii, jj));
                }
            }
        }

        // step 3
        for i in 1..(N - 1) {
            for j in 1..(N - 1) {
                if self.data[i][j] > 9 {
                    self.data[i][j] = 0;
                    self.flash_count += 1;
                }
            }
        }
    }

    fn print(&self) {
        for row in &self.data[1..(N - 1)] {
            println!("{:?}", &row[1..(N - 1)]);
        }
        println!("");
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Failed to read file.");
    let mut grid = Grid::parse_grid(&input);

    grid.print();

    for i in 0..1000000 {
        let old_count = grid.flash_count;
        grid.update();
        if grid.flash_count - old_count == 100 {
            println!("{}", i);
            break;
        }
    }
}
