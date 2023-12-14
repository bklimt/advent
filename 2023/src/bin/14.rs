use anyhow::{anyhow, Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
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
    map: Vec<Vec<char>>,
}

impl Input {
    fn process(&self) -> Result<usize> {
        let mut score = 0;
        let mut limit: Vec<usize> = Vec::new();
        limit.resize(self.map.first().context("input is empty")?.len(), 0);
        for (i, row) in self.map.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                match c {
                    '.' => {}
                    '#' => limit[j] = i + 1,
                    'O' => {
                        score += self.map.len() - limit[j];
                        limit[j] += 1;
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(score)
    }

    fn score(&self) -> Result<usize> {
        let mut score = 0;
        for (i, row) in self.map.iter().enumerate() {
            for &c in row.iter() {
                match c {
                    '.' => {}
                    '#' => {}
                    'O' => {
                        score += self.map.len() - i;
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(score)
    }

    fn print(&self) {
        for row in self.map.iter() {
            let s: String = row.iter().join("");
            println!("{}", s);
        }
        println!("");
    }

    fn tilt_north(&mut self) -> Result<()> {
        let rows = self.map.len();
        let cols = self.map.first().context(anyhow!("empty map"))?.len();

        let mut limit: Vec<usize> = Vec::new();
        limit.resize(cols, 0);

        let m = &mut self.map;
        for i in 0..rows {
            for j in 0..cols {
                let c = m[i][j];
                match c {
                    '.' => {}
                    '#' => limit[j] = i + 1,
                    'O' => {
                        m[i][j] = '.';
                        m[limit[j]][j] = 'O';
                        limit[j] += 1;
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(())
    }

    fn tilt_south(&mut self) -> Result<()> {
        let rows = self.map.len();
        let cols = self.map.first().context(anyhow!("empty map"))?.len();

        let mut limit: Vec<usize> = Vec::new();
        limit.resize(cols, rows - 1);

        let m = &mut self.map;
        for i in (0..rows).rev() {
            for j in 0..cols {
                let c = m[i][j];
                match c {
                    '.' => {}
                    '#' => limit[j] = if i == 0 { 0 } else { i - 1 },
                    'O' => {
                        m[i][j] = '.';
                        m[limit[j]][j] = 'O';
                        limit[j] -= if limit[j] == 0 { 0 } else { 1 };
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(())
    }

    fn tilt_west(&mut self) -> Result<()> {
        let rows = self.map.len();
        let cols = self.map.first().context(anyhow!("empty map"))?.len();

        let mut limit: Vec<usize> = Vec::new();
        limit.resize(rows, 0);

        let m = &mut self.map;
        for j in 0..cols {
            for i in 0..rows {
                let c = m[i][j];
                match c {
                    '.' => {}
                    '#' => limit[i] = j + 1,
                    'O' => {
                        m[i][j] = '.';
                        m[i][limit[i]] = 'O';
                        limit[i] += 1;
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(())
    }

    fn tilt_east(&mut self) -> Result<()> {
        let rows = self.map.len();
        let cols = self.map.first().context(anyhow!("empty map"))?.len();

        let mut limit: Vec<usize> = Vec::new();
        limit.resize(rows, cols - 1);

        let m = &mut self.map;
        for j in (0..cols).rev() {
            for i in 0..rows {
                let c = m[i][j];
                match c {
                    '.' => {}
                    '#' => limit[i] = if j == 0 { 0 } else { j - 1 },
                    'O' => {
                        m[i][j] = '.';
                        m[i][limit[i]] = 'O';
                        limit[i] -= if limit[i] == 0 { 0 } else { 1 };
                    }
                    _ => return Err(anyhow!("invalid char {}", c)),
                }
            }
        }
        Ok(())
    }

    fn cycle(&mut self) -> Result<()> {
        self.tilt_north()?;
        self.tilt_west()?;
        self.tilt_south()?;
        self.tilt_east()?;
        Ok(())
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 'O' {
                    (i, j).hash(&mut hasher);
                }
            }
        }
        hasher.finish()
    }
}

fn read_input(path: &str, _debug: bool) -> Result<Input> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);

    let mut v = Vec::new();

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

        v.push(line.chars().collect_vec());
    }

    Ok(Input { map: v })
}

fn process(args: &Args) -> Result<()> {
    let mut input = read_input(&args.input, args.debug)?;
    println!("ans 1: {}", input.process()?);

    let mut seen = HashMap::new();
    let progress = ProgressBar::new(1000000000);
    let mut i = 0u64;
    while i < 1000000000u64 {
        let h = input.hash();
        if let Some(prev) = seen.get(&h) {
            // We saw a repeat so we can warp forward.
            println!("repeat {} at {}", prev, i);
            let remaining = 1000000000u64 - i;
            let delta = i - prev;
            let repeats = remaining / delta;
            let warp = repeats * delta;
            println!("can jump {}*{} times by {} to {}", repeats, delta, warp, i);
            i += warp;
            progress.inc(warp);
        } else {
            seen.insert(h, i);
        }
        input.cycle()?;
        i += 1;
        progress.inc(1);
    }
    progress.finish();

    if args.debug {
        input.print();
    }
    println!("ans 2: {}", input.score()?);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
