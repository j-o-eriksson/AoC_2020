// AoC 2021 day 13

const N: usize = 1500;

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
    Null,
}

impl Fold {
    fn from_str(s: &str) -> Fold {
        let (fold_str, val_str) = s.split_once("=").unwrap();
        match fold_str {
            "fold along x" => Fold::X(val_str.parse().unwrap()),
            "fold along y" => Fold::Y(val_str.parse().unwrap()),
            _ => Fold::Null,
        }
    }
}

struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn add_point(&mut self, p: (usize, usize)) {
        self.data[p.1][p.0] += 1;
    }

    fn fold_left(&mut self, x: usize) {
        for y in 0..N {
            for dx in 1..(x + 1) {
                let x0 = x - dx;
                let x1 = x + dx;
                self.data[y][x0] += self.data[y][x1];
                self.data[y][x1] = 0;
            }
        }
    }

    fn fold_up(&mut self, y: usize) {
        for x in 0..N {
            for dy in 1..(y + 1) {
                let y0 = y - dy;
                let y1 = y + dy;
                self.data[y0][x] += self.data[y1][x];
                self.data[y1][x] = 0;
            }
        }
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::X(x) => self.fold_left(x),
            Fold::Y(y) => self.fold_up(y),
            Fold::Null => (),
        }
    }
}

fn from_string(s: &str) -> (usize, usize) {
    let (x, y) = s.split_once(",").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn main() {
    let (a, b) = include_str!("input").split_once("\n\n").unwrap();

    let mut grid = Grid { data: vec![vec![0; N]; N] };
    a.lines().map(from_string).for_each(|p| grid.add_point(p));
    // let l1 = b.lines().map(Fold::from_str).next().unwrap();
    b.lines().map(Fold::from_str).for_each(|f| grid.fold(f));

    // println!("{:?}", grid.data);

    // let count = grid.data.into_iter().map(|line| line.filter(|val| val != 0).count()).sum();
    let mut count = 0;
    for line in grid.data {
        for val in line {
            if val != 0 {
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}
