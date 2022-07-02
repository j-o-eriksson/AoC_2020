// AoC 2021 day 9

use std::collections::HashSet;
use std::fs;

const N: usize = 150;
type Grid = [[u8; N]; N];

fn parse_grid(s: &str) -> Grid {
    let mut grid: Grid = [[9; N]; N];

    let mut i = 2;
    for row in s.split("\n") {
        let mut j = 2;
        for val in row.chars().map(|c| c.to_digit(10).unwrap()) {
            grid[i][j] = val as u8;
            j += 1;
        }
        i += 1;
    }

    return grid;
}

fn pred(grid: &Grid, i: usize, j: usize, p: &(i32, i32)) -> bool {
    let ii = (i as i32 + p.0) as usize;
    let jj = (j as i32 + p.1) as usize;
    grid[i][j] >= grid[ii][jj]
}

fn find_minima(grid: &Grid) -> Vec<(usize, usize)> {
    let neighbor_steps = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut minima = Vec::new();
    for i in 1..(N - 1) {
        for j in 1..(N - 1) {
            if !neighbor_steps.iter().any(|p| pred(grid, i, j, p)) {
                minima.push((i, j));
            }
        }
    }

    return minima;
}

fn explore_basin(grid: &Grid, minimum: (usize, usize)) -> usize {
    let neighbor_steps = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut unvisited = vec![(minimum.0 as i32, minimum.1 as i32)];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while !unvisited.is_empty() {
        let location = unvisited.pop().unwrap();
        for step in &neighbor_steps {
            let l = (location.0 + step.0, location.1 + step.1);
            if !visited.contains(&l) && grid[l.0 as usize][l.1 as usize] < 9 {
                unvisited.push(l);
            }
        }
        visited.insert(location);
    }

    visited.len()
}

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file.");
    let grid = parse_grid(&contents);
    let minima = find_minima(&grid);

    // part 1
    let risk_level: u32 = minima.iter().map(|p| 1 + grid[p.0][p.1] as u32).sum();
    println!("part 1 answer: {}", risk_level);

    // part 2
    let mut basin_sizes: Vec<usize> = minima.iter().map(|p| explore_basin(&grid, *p)).collect();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "part 2 answer: {}",
        basin_sizes[..3].iter().product::<usize>()
    );
}
