use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coord(usize, usize);

struct Grid {
    vals: Vec<Vec<u32>>,
}

fn new_map(rows: usize, cols: usize) -> Grid {
    return Grid {
        vals: vec![vec![0; cols]; rows],
    };
}

fn between(low: usize, val: usize, high: usize) -> bool {
    return low <= val && val <= high;
}

impl Grid {
    fn rows(&self) -> usize { return self.vals.len(); }
    fn cols(&self) -> usize { return self.vals.first().unwrap().len(); }

    fn get_mut(&mut self, coord: Coord) -> &mut u32 {
        return self.vals.get_mut(coord.0).unwrap().get_mut(coord.1).unwrap();
    }
    fn get(&self, coord: Coord) -> u32 {
        return *self.vals.get(coord.0).unwrap().get(coord.1).unwrap();
    }
    fn neighbours(&self, coord: Coord) -> Vec<Coord> {
        let mut result = Vec::new();
        for (rd, cd) in [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (2, 1), (2, 0), (1, 0)] {
            if !between(1, coord.0 + rd, self.rows()) || !between(1, coord.1 + cd, self.cols()) {
                continue;
            }
            let new_coord = Coord(coord.0 + rd - 1, coord.1 + cd - 1);
            result.push(new_coord);
        }
        return result;
    }
}

fn print_state(energies: &Grid) {
    for r in 0..10 {
        println!("{}", energies.vals[r].iter().fold(
            String::new(),
            |mut acc, ele| {

                if *ele > 9 {
                    acc+="*";acc } else {
                    acc += &*ele.to_string();acc
                }
            }
        ));
    }
    println!("");
}

fn main() {
    let mut energies = new_map(10, 10);
    for (r, line) in fs::read_to_string("input.txt").unwrap().lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            let coord = Coord(r, c);
            let digit = char.to_digit(10).unwrap();
            *energies.get_mut(coord) = digit;
        }
    }


    println!("Before any steps:");
    print_state(&energies);
    let mut num_flashed = 0;
    for step in 1..1031 {
        for r in 0..10 {
            for c in 0..10 {
                let coord = Coord(r, c);
                *energies.get_mut(coord) += 1;
            }
        }

        let mut flashed = HashSet::new();

        loop {
            let mut new_flashes = false;
            for r in 0..10 {
                for c in 0..10 {
                    let coord = Coord(r, c);
                    if energies.get(coord) > 9 && !flashed.contains(&coord) {
                        flashed.insert(coord);
                        let neighbours = energies.neighbours(coord);
                        for neighbour in neighbours {
                            *energies.get_mut(neighbour) += 1;
                        }
                        new_flashes = true;
                    }
                }
            }
            if !new_flashes { break; }
        }

        for &coord in flashed.iter() {
            *energies.get_mut(coord) = 0;
        }
        num_flashed += flashed.len();

        println!("After step {}:", step);
        print_state(&energies);

        if flashed.len() == 100 {
            println!("All simultaneous on step {}", step);
            break;
        }
    }

    println!("{}", num_flashed);
}
