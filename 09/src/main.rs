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

fn count_basin(basin_map: &mut Map, coord: Coord) -> usize {
    if basin_map.get(coord) == 0 {
        return 0;
    }

    *basin_map.get_mut(coord) = 0;
    let mut sum = 1;
    let neighbours = basin_map.neighbours(coord);
    for neighbour in neighbours {
        sum += count_basin(basin_map, neighbour);
    }
    return sum;
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

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

    for r in 0..rows {
        for c in 0..cols {
            let coord = Coord(r, c);
            let this_height = height_map.get(coord);
            if is_low_point(&height_map, coord) {
                let risk_level = this_height + 1;
                risk_level_sum += risk_level;
            }

            // Assign a basin to the cell
            *basin_map.get_mut(coord) = if this_height == 9 { 0 } else { 1 };
        }
    }
    println!("{}", risk_level_sum); // part 1


    // Count basins
    let mut top_counts = vec![0; 3];
    for r in 0..rows {
        for c in 0..cols {
            let coord = Coord(r, c);
            let count = count_basin(&mut basin_map, coord);
            if count > top_counts[0] {
                top_counts[0] = count;
                top_counts.sort();
            }
        }
    }

    println!("{}", top_counts[0] * top_counts[1] * top_counts[2]);
}
