use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashSet;
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

fn process_input(path: &str) -> Result<u32> {
    let mut tail = (0, 0);
    let mut head = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);

    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
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

        let command = line
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow!("invalid command: {}", line))?;

        if line
            .chars()
            .nth(1)
            .ok_or_else(|| anyhow!("invalid command: {}", line))?
            != ' '
        {
            return Err(anyhow!("invalid command: {}", line));
        }

        let (_, amount_str) = line.split_at(2);
        let amount = amount_str
            .parse::<i32>()
            .with_context(|| format!("invalid amount: {:?} in {}", amount_str, line))?;

        for _ in 0..amount {
            match command {
                'R' => {
                    head.0 = head.0 + 1;
                    if tail.0 < head.0 - 1 {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                    }
                }
                'L' => {
                    head.0 = head.0 - 1;
                    if tail.0 > head.0 + 1 {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                    }
                }
                'U' => {
                    head.1 = head.1 - 1;
                    if tail.1 > head.1 + 1 {
                        tail.1 = head.1 + 1;
                        tail.0 = head.0;
                    }
                }
                'D' => {
                    head.1 = head.1 + 1;
                    if tail.1 < head.1 - 1 {
                        tail.1 = head.1 - 1;
                        tail.0 = head.0;
                    }
                }
                _ => {
                    return Err(anyhow!("invalid input: {}", line));
                }
            };
            visited.insert(tail);
        }

        // println!("{} {}", command, amount);
    }
    Ok(visited.len() as u32)
}

fn process(path: &str, _part2: bool) -> Result<()> {
    let visited = process_input(path)?;
    println!("part1 = {}", visited);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
