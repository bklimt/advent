use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::fold;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,
}

fn read_input(path: &str) -> Result<Vec<Vec<u32>>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut data: Vec<Vec<u32>> = Vec::new();
    let mut width: Option<usize> = None;
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        let mut row: Vec<u32> = Vec::new();
        for c in line.as_bytes() {
            row.push((*c as u32) - ('0' as u32));
        }
        if width.is_some() {
            if row.len() != width.unwrap() {
                return Err(anyhow!(
                    "invalid row length: {} vs {}",
                    row.len(),
                    width.unwrap()
                ));
            }
        } else {
            width = Some(row.len());
        }
        data.push(row);
    }
    if data.len() == 0 {
        return Err(anyhow!("data is empty"));
    }
    Ok(data)
}

fn compute_visible(data: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let height = data.len();
    let width = data[0].len();
    let mut result: Vec<Vec<bool>> = Vec::new();
    for _ in 0..height {
        let mut row: Vec<bool> = Vec::new();
        row.resize(width, false);
        result.push(row);
    }
    for i in 0..height {
        for j in 0..width {
            // Check from the left.
            let mut visible_left = true;
            for k in 0..j {
                if data[i][k] >= data[i][j] {
                    visible_left = false;
                    break;
                }
            }
            if visible_left {
                result[i][j] = true;
                continue;
            }

            // Check from the right.
            let mut visible_right = true;
            for k in (j + 1)..width {
                if data[i][k] >= data[i][j] {
                    visible_right = false;
                    break;
                }
            }
            if visible_right {
                result[i][j] = true;
                continue;
            }

            // Check from the top.
            let mut visible_top = true;
            for k in 0..i {
                if data[k][j] >= data[i][j] {
                    visible_top = false;
                    break;
                }
            }
            if visible_top {
                result[i][j] = true;
                continue;
            }

            // Check from the bottom.
            let mut visible_bottom = true;
            for k in (i + 1)..height {
                if data[k][j] >= data[i][j] {
                    visible_bottom = false;
                    break;
                }
            }
            if visible_bottom {
                result[i][j] = true;
                continue;
            }
        }
    }
    result
}

fn print_visibility(data: &Vec<Vec<bool>>) {
    for row in data.iter() {
        for col in row.iter() {
            print!("{}", if *col { "#" } else { " " });
        }
        println!("");
    }
}

fn count_visible(data: &Vec<Vec<bool>>) -> i32 {
    fold(data, 0, |c, row| {
        c + fold(row, 0, |c, v| c + if *v { 1 } else { 0 })
    })
}

fn process(path: &str, _part2: bool) -> Result<()> {
    let data = read_input(path)?;
    let visibility = compute_visible(&data);
    print_visibility(&visibility);
    let visible = count_visible(&visibility);
    println!("visible = {}", visible);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
