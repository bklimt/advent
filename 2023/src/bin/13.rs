use anyhow::{anyhow, Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

fn find_mirror1(v: &Vec<u64>) -> Option<usize> {
    /*
     * n=10
     *
     * split=0
     * (0, 1)
     *  |
     * 0 1 2 3 4 5 6 7 8 9
     *
     * split=1
     * (1, 2), (0, 3)
     *    |
     * 0 1 2 3 4 5 6 7 8 9
     *
     * split=8
     * (8, 9)
     *                  |
     * 0 1 2 3 4 5 6 7 8 9
     *
     */
    let n = v.len() as i64;
    for split in 0..(n - 1) {
        let mut matches = true;
        for i in 0..n {
            let a = split - i;
            let b = split + i + 1;
            if a < 0 || b >= n {
                break;
            }
            if v[a as usize] != v[b as usize] {
                matches = false;
                break;
            }
        }
        if matches {
            return Some((split + 1) as usize);
        }
    }
    None
}

fn find_mirror2(v: &Vec<u64>) -> Option<usize> {
    let n = v.len() as i64;
    for split in 0..(n - 1) {
        let mut bits_wrong = 0u32;
        for i in 0..n {
            let a = split - i;
            let b = split + i + 1;
            if a < 0 || b >= n {
                break;
            }
            bits_wrong += (v[a as usize] ^ v[b as usize]).count_ones();
            if bits_wrong > 1 {
                break;
            }
        }
        if bits_wrong == 1 {
            return Some((split + 1) as usize);
        }
    }
    None
}

fn find_mirror(v: &Vec<u64>, part2: bool) -> Option<usize> {
    if part2 {
        find_mirror2(v)
    } else {
        find_mirror1(v)
    }
}

impl Grid {
    fn from_text(text: &Vec<String>) -> Result<Self> {
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        for line in text.iter() {
            let mut row = 0u64;
            for (i, c) in line.chars().enumerate() {
                if i >= cols.len() {
                    cols.push(0);
                }
                let col = cols.get_mut(i).expect("cols.len() > i");

                let bit = match c {
                    '.' => 0,
                    '#' => 1,
                    _ => return Err(anyhow!("invalid character: {}", c)),
                };

                row *= 2;
                row += bit;

                *col *= 2;
                *col += bit;
            }
            rows.push(row);
        }
        Ok(Grid { rows, cols })
    }

    fn find_mirror(&self, part2: bool) -> Result<usize> {
        if let Some(n) = find_mirror(&self.rows, part2) {
            Ok(100 * n)
        } else if let Some(n) = find_mirror(&self.cols, part2) {
            Ok(n)
        } else {
            Err(anyhow!("no answer!"))
        }
    }
}

fn read_input(path: &str, _debug: bool) -> Result<Vec<Grid>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);

    let mut grids = Vec::new();
    let mut lines = Vec::new();

    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if lines.len() > 0 {
                let grid = Grid::from_text(&lines)?;
                grids.push(grid);
                lines = Vec::new();
            }

            if n == 0 {
                break;
            }
            continue;
        }

        lines.push(line.to_owned());
    }

    Ok(grids)
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(&args.input, args.debug)?;
    let progress = ProgressBar::new(input.len() as u64 * 2);
    let mut total1 = 0usize;
    let mut total2 = 0usize;
    for grid in input.iter() {
        if args.debug {
            println!("grid: {:?}", grid);
        }
        total1 += grid.find_mirror(false)?;
        progress.inc(1);
        total2 += grid.find_mirror(true)?;
        progress.inc(1);
    }
    progress.finish();
    println!("ans1 = {}", total1);
    println!("ans2 = {}", total2);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
