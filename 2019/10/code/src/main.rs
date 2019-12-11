
use std::env;
use std::f32;
use std::fs;

fn print_map(map: &Vec<Vec<i32>>) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                print!(".");
            } else {
                // print!("{}", map[row][col]);
                print!("#");
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

    let row = 13;
    let col = 11;
    let mut prev_angle = -1.0;
    let mut steps = 0;
    println!("{},{}:", row, col);            
    loop {
        let mut min_angle = 8.0;
        let mut min_distance = 0;
        let mut min_row = 0;
        let mut min_col = 0;
        let mut found = false;
        // Iterate over all the other asteroids.
        for r in 0..rows {
            for c in 0..cols {
                if r == row && c == col {
                    continue;
                }
                if map[r][c] == 1 {
                    // Check if it's visible.

                    // Create a normalized vector pointing at the target.
                    let dy = (r as i32) - (row as i32);
                    let dx = (c as i32) - (col as i32);
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

                    if angle > prev_angle {
                        if !found || angle < min_angle || (min_angle == angle && distance < min_distance) {
                            min_angle = angle;
                            min_distance = distance;
                            min_row = r;
                            min_col = c;
                            found = true;
                        }
                    }
                }
            }
        }
        if !found {
            break;
        }
        steps = steps + 1;
        println!("{}: destroying [{},{}] at angle={}, distance={}", steps, min_col, min_row, min_angle, min_distance);
        map[min_row][min_col] = 0;
        prev_angle = min_angle;
        //print_map(&map);
        //println!("");
    }
    // print_map(&result);

    // println!("max: {} at [{}, {}]", max, max_row, max_col);
}

// 230
