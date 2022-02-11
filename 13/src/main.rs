use std::cmp::max;
use std::{fmt, fs};

struct DotMatrix {
    matrix: Vec<Vec<bool>>,
    xs: usize,
    ys: usize,
}

impl DotMatrix {
    fn new() -> DotMatrix {
        return DotMatrix {
            matrix: vec![vec![false]],
            xs: 1,
            ys: 1,
        };
    }

    fn add(&mut self, x: usize, y: usize) {
        self.enlarge(x, y);
        self.matrix[x][y] = true;
    }

    fn enlarge(&mut self, x: usize, y: usize) {
        self.resize(max(self.xs, x + 1), max(self.ys, y + 1));
    }

    fn resize(&mut self, x: usize, y: usize) {
        self.xs = x;
        self.ys = y;

        self.matrix.resize(self.xs, Vec::new());
        for xi in 0..(self.xs) {
            self.matrix[xi].resize(self.ys, false);
        }
    }

    fn fold_x(&mut self, x: usize) {
        for xi in 0..x {
            for yi in 0..(self.ys) {
                self.matrix[xi][yi] |= self.matrix[self.xs - xi-1][yi];
            }
        }
        self.resize(x , self.ys);
    }

    fn fold_y(&mut self, y: usize) {
        for xi in 0..(self.xs) {
            for yi in 0..y {
                self.matrix[xi][yi] |= self.matrix[xi][self.ys - yi-1];
            }
        }
        self.resize(self.xs, y );
    }

    fn count(&self) -> usize {
        self.matrix.iter().fold(0,
            |sum, col| col.iter().fold(sum,
                   |sum, v| sum + if *v { 1 } else { 0 }))
    }
}

// impl fmt::Display for DotMatrix {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.matrix.jo())
//     }
// }

fn main() {
    let mut matrix = DotMatrix::new();

    let mut folds_mode = false;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        if line.len() == 0 {
            folds_mode = true;
        } else if folds_mode == false {
            let vec: Vec<&str> = line.splitn(2, ",").collect();
            let (x, y) = (vec[0].parse::<usize>().unwrap(), vec[1].parse::<usize>().unwrap());
            matrix.add(x, y);
        } else {
            let vec: Vec<&str> = line.splitn(2, "=").collect();
            let num = vec[1].parse::<usize>().unwrap();
            if line.starts_with("fold along x=") {
                matrix.fold_x(num);
            } else if line.starts_with("fold along y=") {
                matrix.fold_y(num);
            }
            println!("{}", matrix.count());
            break;
        }
    }

    // println!("{}", matrix.count());
}
