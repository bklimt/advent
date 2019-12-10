
use std::collections::HashSet;
use std::fs;
use vector2d::Vector2D;

extern crate num_rational;

fn print_map(map: &Vec<Vec<i32>>) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                print!(".");
            } else {
                print!("{}", map[row][col]);
            }
        }
        println!("");
    }
}

fn main() {
    let text = fs::read_to_string("input1.txt")
        .expect("unable to read file");

    // Determine the dimensions.
    let mut rows = 0;
    let mut cols = 0;
    let lines = text.split("\n");
    for line in lines {
        rows = rows + 1;
        for _ in line.chars() {
            if rows == 1 {
                cols = cols + 1;
            }
        }
    }
    println!("rows: {}, cols: {}", rows, cols);

    // Make an 2-dimensional array.
    let mut map = vec![vec![0; cols]; rows];
    let mut row = 0;
    let lines = text.split("\n");
    for line in lines {
        let mut col = 0;
        for c in line.chars() {
            if c == '#' {
                map[row][col] = 1;
            }
            col = col + 1;
        }
        row = row + 1;
    }
    print_map(&map);
    println!("");

    // Build up the result.
    let mut result = vec![vec![0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            println!("{},{}:", row, col);            
            if map[row][col] == 1 {
                let mut seen = HashSet::new();
                // Iterate over all the other asteroids.
                for r in 0..rows {
                    for c in 0..cols {
                        if map[r][c] == 1 {
                            // Check if it's visible.
                            let dy = (r as i32) - (row as i32);
                            let dx = (c as i32) - (col as i32);
                            // Normalize the vector.
                            println!("  {},{}: vec: {:?}", r, c, t);
                            if !seen.contains(&t) {
                                seen.insert(t);
                                result[row][col] = result[row][col] + 1;
                            }
                        }
                    }
                }
            }
        }
    }
    print_map(&result);

    // println!("{:?}", map);
}
