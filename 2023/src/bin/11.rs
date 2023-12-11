use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashSet;
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
struct Input {
    rows: usize,
    columns: usize,
    galaxies: Vec<(usize, usize)>,
}

impl Input {
    fn read(path: &str, _debug: bool) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
        let mut r = BufReader::new(file);

        let mut rows: usize = 0;
        let mut columns: usize = 0;
        let mut galaxies: Vec<(usize, usize)> = Vec::new();
        let mut rows_seen: HashSet<usize> = HashSet::new();
        let mut columns_seen: HashSet<usize> = HashSet::new();

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

            for (i, c) in line.chars().enumerate() {
                columns = columns.max(i + 1);
                if c == '#' {
                    galaxies.push((rows, i));
                    rows_seen.insert(rows);
                    columns_seen.insert(i);
                }
            }
            rows += 1;
        }

        for row in (0..rows).rev() {
            if !rows_seen.contains(&row) {
                for galaxy in galaxies.iter_mut() {
                    if galaxy.0 > row {
                        galaxy.0 += 1;
                    }
                }
                rows += 1;
            }
        }
        for col in (0..columns).rev() {
            if !columns_seen.contains(&col) {
                for galaxy in galaxies.iter_mut() {
                    if galaxy.1 > col {
                        galaxy.1 += 1;
                    }
                }
                columns += 1;
            }
        }

        Ok(Input {
            rows,
            columns,
            galaxies,
        })
    }

    fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.columns {
                let is_galaxy = self.galaxies.binary_search(&(r, c)).is_ok();
                print!("{}", if is_galaxy { '#' } else { '.' });
            }
            println!("");
        }
    }

    fn part1(&self, debug: bool) -> Result<usize> {
        let n = self.galaxies.len();
        let mut adj = vec![vec![0usize; n]; n];

        // Initialize every pair with the manhattan distance.
        for i in 0..n {
            adj[i][i] = 0;
            let gi = self.galaxies.get(i).expect("in range");
            for j in 0..i {
                let gj = self.galaxies.get(j).expect("in range");
                let dy = gi.0.abs_diff(gj.0);
                let dx = gi.1.abs_diff(gj.1);
                let d = dx + dy;
                adj[i][j] = d;
                adj[j][i] = d;
            }
        }

        // Run Floyd's algorithm.
        for k in 0..n {
            for i in 0..n {
                for j in 0..i {
                    let d1 = adj[i][j];
                    let d2 = adj[i][k] + adj[k][j];
                    if d2 < d1 {
                        adj[i][j] = d2;
                        adj[j][i] = d2;
                    }
                }
            }
        }

        // Compute the answer.
        let mut total: usize = 0;
        for i in 0..n {
            for j in 0..i {
                if debug {
                    println!("d({}, {}) = {}", i, j, adj[i][j]);
                }
                total += adj[i][j];
            }
        }

        Ok(total)
    }

    fn part2(&self, _debug: bool) -> Result<i64> {
        Ok(0)
    }
}

fn process(args: &Args) -> Result<()> {
    let input = Input::read(&args.input, args.debug)?;
    if args.debug {
        input.print();
        println!("");
    }
    println!("ans1: {}", input.part1(args.debug)?);
    println!("ans2: {}", input.part2(args.debug)?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
