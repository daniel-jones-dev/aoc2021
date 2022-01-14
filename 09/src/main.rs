use std::cmp::{max, min};
use std::fs;

// #[derive(Clone, Copy)]
// struct Coord {
//     row: usize,
//     col: usize,
// }

#[derive(Clone, Copy)]
struct Coord(usize, usize);

struct Map {
    vals: Vec<Vec<u32>>,
}

fn new_map(rows: usize, cols: usize) -> Map {
    return Map {
        vals: vec![vec![0; cols]; rows],
    };
}

impl Map {
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
        let (r, c) = (coord.0, coord.1);
        if r > 0 {
            result.push(Coord(r - 1, c));
        }
        if c > 0 {
            result.push(Coord(r, c - 1));
        }
        if r + 1 < self.rows() {
            result.push(Coord(r + 1, c));
        }
        if c + 1 < self.cols() {
            result.push(Coord(r, c + 1));
        }
        return result;
    }
}


fn is_low_point(height_map: &Map, coord: Coord) -> bool {
    let this_height = height_map.get(coord);
    let neighbours = height_map.neighbours(coord);
    return !neighbours.iter().any(|neighbour| height_map.get(*neighbour) <= this_height);
}

fn first_neighbour_basin(basin_map: &Map, coord: Coord, ignore: Option<u32>) -> u32 {
    let neighbours = basin_map.neighbours(coord);
    for neighbour in neighbours {
        let neighbour_basin = basin_map.get(neighbour);
        if neighbour_basin != 0 && neighbour_basin != ignore.unwrap_or(0) { return neighbour_basin; }
    }
    return 0;
}

fn reassign_basin(basin_map: &mut Map, from: u32, to: u32) {
    for r in 0..basin_map.rows() {
        for c in 0..basin_map.cols() {
            let coord = Coord(r, c);
            if basin_map.get(coord) == from {
                *basin_map.get_mut(coord) = to;
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("example.txt").unwrap();

    let cols = contents.lines().next().unwrap().len();
    let rows = contents.lines().count();

    let mut height_map = new_map(rows, cols);

    for (r, line) in contents.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            let coord = Coord(r, c);
            *height_map.get_mut(coord) = char.to_digit(10).unwrap();
        }
    }

    let mut risk_level_sum = 0;
    let mut basin_map = new_map(rows, cols);
    let mut max_basin = 0;

    for r in 0..rows {
        for c in 0..cols {
            let coord = Coord(r, c);
            let this_height = height_map.get(coord);
            if is_low_point(&height_map, coord) {
                let risk_level = this_height + 1;
                risk_level_sum += risk_level;
            }

            // Assign a basin to the cell
            if this_height == 9 {
                *basin_map.get_mut(coord) = 0;
            } else {
                let neighbour_basin = first_neighbour_basin(&basin_map, coord, None);
                if neighbour_basin != 0 {
                    *basin_map.get_mut(coord) = neighbour_basin;
                } else {
                    // Assign new basin
                    max_basin += 1;
                    *basin_map.get_mut(coord) = max_basin;
                }
            }
        }
    }
    println!("{}", risk_level_sum); // part 1

    // Reassign neighbouring basins
    for r in 0..rows {
        for c in 0..cols {
            let coord = Coord(r, c);
            let this_basin = basin_map.get(coord);
            if this_basin != 0 {
                let neighbour_basin = first_neighbour_basin(&basin_map, coord, Option::from(this_basin));
                if neighbour_basin != 0 {
                    reassign_basin(&mut basin_map, max(this_basin, neighbour_basin), min(this_basin, neighbour_basin));
                }
            }
        }
    }

    struct CountBasinNum(usize, u32);
    let mut basin_sums = Vec::new();

    // Count basin sizes
    for b in 1..max_basin + 1 {
        let mut sum = 0;
        for r in 0..rows {
            for c in 0..cols {
                let coord = Coord(r, c);
                if basin_map.get(coord) == b {
                    sum += 1;
                }
            }
        }
        basin_sums.push(CountBasinNum(sum, b));
    }
    basin_sums.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{}", basin_sums[0].0 * basin_sums[1].0 * basin_sums[2].0);
}
