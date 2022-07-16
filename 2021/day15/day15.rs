// AoC 2022 day 15

type Grid = Vec<Vec<u16>>;

fn shape(grid: &Grid) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn step(
    x: usize,
    y: usize,
    (n, m): (usize, usize),
    (dx, dy): (i32, i32),
) -> Option<(usize, usize)> {
    let px = x as i32 + dx;
    let py = y as i32 + dy;

    if (px < 0 || px >= m as i32) || (py < 0 || py >= n as i32) {
        return None;
    }
    Some((px as usize, py as usize))
}

fn explore_neighborhood(x: usize, y: usize, risks: &Grid, total_risks: &mut Grid) {
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let risk = total_risks[y][x];
    for neighbor in &neighbors {
        if let Some((px, py)) = step(x, y, shape(risks), neighbor.clone()) {
            let next_risk = risk + risks[py][px];
            if next_risk < total_risks[py][px] {
                total_risks[py][px] = next_risk;
                explore_neighborhood(px, py, risks, total_risks);
            }
        }
    }
}

fn next_grid(prev: &Grid, i: usize) -> Grid {
    let mut next = prev.clone();
    for y in 0..next.len() {
        for x in 0..next[0].len() {
            let z = next[y][x] + i as u16;
            if z > 9 {
                next[y][x] = z % 9;
            } else {
                next[y][x] = z;
            }
        }
    }
    next
}

fn fill_grid(big: &mut Grid, small: &Grid, i: usize, j: usize) {
    let next = next_grid(small, i + j);
    let (n, m) = shape(small);

    let y0 = i * n;
    let x0 = j * m;

    for x in 0..n {
        for y in 0..m {
            big[y0 + y][x0 + x] = next[y][x];
        }
    }
}

fn make_5x5(grid: &Grid) -> Grid {
    let (n, m) = shape(grid);
    let mut grid_5x5 = vec![vec![0; m * 5]; n * 5];

    for i in 0..5 {
        for j in 0..5 {
            fill_grid(&mut grid_5x5, &grid, i, j);
        }
    }
    grid_5x5
}

fn run(risks: &Grid) {
    let (n, m) = shape(risks);
    let mut total_risks = vec![vec![10000; m]; n];
    total_risks[0][0] = 0;
    explore_neighborhood(0, 0, risks, &mut total_risks);
    println!("{}", total_risks.last().unwrap().last().unwrap());
}

fn main() {
    let risks: Grid = include_str!("test_input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    // part 1
    run(&risks);

    // part2
    let big = make_5x5(&risks);
    run(&big);
}
