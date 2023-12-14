use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
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
    let input = read_input(&args.input, args.debug)?;
    println!("ans: {}", input.process()?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
