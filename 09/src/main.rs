use std::fs;

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

    for r in 0..rows {
        for c in 0..cols {
            let this_height = height_map[r][c];
            if r > 0 && height_map[r - 1][c] <= this_height { continue; }
            if c > 0 && height_map[r][c - 1] <= this_height { continue; }
            if r + 1 < rows && height_map[r + 1][c] <= this_height { continue; }
            if c + 1 < cols && height_map[r][c + 1] <= this_height { continue; }

            let risk_level = this_height +1;
            risk_level_sum += risk_level;
        }
    }

    println!("{}", risk_level_sum);
}
