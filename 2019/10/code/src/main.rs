
extern crate num_rational;

use std::collections::HashSet;
use std::env;
use std::f32;
use std::fs;

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
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let text = fs::read_to_string(filename)
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
    let mut max = 0;
    let mut max_row = 0;
    let mut max_col = 0;
    let mut result = vec![vec![0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            // println!("{},{}:", row, col);            
            if map[row][col] == 1 {
                let mut seen = HashSet::new();
                // Iterate over all the other asteroids.
                for r in 0..rows {
                    for c in 0..cols {
                        if r == row && c == col {
                            continue;
                        }
                        if map[r][c] == 1 {
                            // Check if it's visible.

                            // Create a normalized vector pointing at the target.
                            let mut dy = (r as i32) - (row as i32);
                            let mut dx = (c as i32) - (col as i32);
                            // println!("       dx: {}, dy: {}", dx, dy);

                            // Determine the distance for sorting.
                            let distance = dx * dx + dy * dy;

                            // Determine the angle for sorting.
                            let angle = if dx == 0 && dy == 0 {
                                0.0
                            } else if dx == 0 {
                                if dy < 0 {
                                    0.0
                                } else {
                                    f32::consts::PI
                                }
                            } else if dy == 0 {
                                if dx < 0 {
                                    3.0 * f32::consts::PI / 2.0
                                } else {
                                    f32::consts::PI / 2.0
                                }
                            } else if dx > 0 {
                                if dy > 0 {
                                    // dx > 0
                                    // dy > 0
                                    ((dy as f32)/(dx as f32)).atan() + f32::consts::PI / 2.0
                                } else {
                                    // dx > 0
                                    // dy < 0
                                    ((dx as f32)/(-dy as f32)).atan()
                                }
                            } else {
                                if dy > 0 {
                                    // dx < 0
                                    // dy > 0
                                    ((-dx as f32)/(dy as f32)).atan() + f32::consts::PI
                                } else {
                                    // dx < 0
                                    // dy < 0
                                    ((dy as f32)/(dx as f32)).atan() + 3.0 * f32::consts::PI / 2.0
                                }
                            };
                            // println!("        0: {}", angle);

                            let signx = dx.signum();
                            let signy = dy.signum();
                            // println!("       sx: {}, sy: {}", signx, signy);
                            dx = signx * dx;
                            dy = signy * dy;
                            if dx == 0 {
                                dy = 1;
                            } else if dy == 0 {
                                dx = 1;
                            } else {
                                let r = num_rational::Ratio::new(dx, dy);
                                dx = *r.numer();
                                dy = *r.denom();
                                // println!("      *dx: {},*dy: {}", dx, dy);
                            }
                            dx = signx * dx;
                            dy = signy * dy;
                            // println!("      +dx: {},+dy: {}", dx, dy);
                            let norm_vec = (dx, dy);

                            // println!("  {},{}: vec: {:?}", r, c, t);
                            if !seen.contains(&norm_vec) {
                                seen.insert(norm_vec);
                                result[row][col] = result[row][col] + 1;
                            }
                        }
                    }
                }
                let visible = result[row][col];
                if visible > max {
                    max = visible;
                    max_row = row;
                    max_col = col;
                }
            }
        }
    }
    print_map(&result);

    println!("max: {} at [{}, {}]", max, max_row, max_col);
}

// 230
