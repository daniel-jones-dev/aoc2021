use std::cmp::{max, min};
use std::fs;

fn is_low_point(height_map: &Vec<Vec<u32>>, r: usize, c: usize, rows: usize, cols: usize) -> bool {
    let this_height = height_map[r][c];
    if r > 0 && height_map[r - 1][c] <= this_height { return false; }
    if c > 0 && height_map[r][c - 1] <= this_height { return false; }
    if r + 1 < rows && height_map[r + 1][c] <= this_height { return false; }
    if c + 1 < cols && height_map[r][c + 1] <= this_height { return false; }
    return true;
}

fn first_neighbour_basin(basin_map: &Vec<Vec<u32>>, r: usize, c: usize, rows: usize, cols: usize, ignore: Option<u32>) -> u32 {
    if r > 0 {
        let up = basin_map[r - 1][c];
        if up != 0 && up != ignore.unwrap_or(0) { return up; }
    }
    if c > 0 {
        let left = basin_map[r][c - 1];
        if left != 0 && left != ignore.unwrap_or(0) { return left; }
    }
    if r + 1 < rows {
        let down = basin_map[r + 1][c];
        if down != 0 && down != ignore.unwrap_or(0) { return down; }
    }
    if c + 1 > cols {
        let right = basin_map[r][c + 1];
        if right != 0 && right != ignore.unwrap_or(0) { return right; }
    }
    return 0;
}

fn reassign_basin(basin_map: &mut Vec<Vec<u32>>, from: u32, to: u32, rows: usize, cols: usize) {
    for r in 0..rows {
        for c in 0..cols {
            if basin_map[r][c] == from {
                basin_map[r][c] = to;
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let cols = contents.lines().next().unwrap().len();
    let rows = contents.lines().count();

    let mut height_map = vec![vec![0; cols]; rows];

    for (row, line) in contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            height_map[row][col] = char.to_digit(10).unwrap();
        }
    }

    let mut risk_level_sum = 0;
    let mut basin_map = vec![vec![0; cols]; rows];
    let mut max_basin = 0;

    for r in 0..rows {
        for c in 0..cols {
            let this_height = height_map[r][c];
            if is_low_point(&height_map, r, c, rows, cols) {
                let risk_level = this_height + 1;
                risk_level_sum += risk_level;
            }

            // Assign a basin to the cell
            if this_height == 9 {
                basin_map[r][c] = 0;
            } else {
                let neighbour_basin = first_neighbour_basin(&basin_map, r, c, rows, cols, None);
                if neighbour_basin != 0 {
                    basin_map[r][c] = neighbour_basin;
                } else {
                    // Assign new basin
                    max_basin += 1;
                    basin_map[r][c] = max_basin;
                }
            }
        }
    }
    println!("{}", risk_level_sum); // part 1

    // Reassign neighbouring basins
    for r in 0..rows {
        for c in 0..cols {
            let this_basin = basin_map[r][c];
            if this_basin != 0 {
                let neighbour_basin = first_neighbour_basin(&basin_map, r, c, rows, cols, Option::from(this_basin));
                if neighbour_basin != 0 {
                    reassign_basin(&mut basin_map, max(this_basin, neighbour_basin), min(this_basin, neighbour_basin), rows, cols);
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
                if basin_map[r][c] == b {
                    sum += 1;
                }
            }
        }
        basin_sums.push(CountBasinNum(sum, b));
    }
    basin_sums.sort_by(|a, b| b.0.cmp(&a.0));

    println!("{}", basin_sums[0].0 * basin_sums[1].0 * basin_sums[2].0);
}
