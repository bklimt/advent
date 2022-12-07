use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::{HashMap, VecDeque};
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

pub fn read_data(path: &str) -> Result<String> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut line = String::new();
    r.read_line(&mut line).unwrap();
    let line = line.trim();
    Ok(line.to_string())
}

pub fn process(path: &str, span: usize) -> Result<()> {
    let data = read_data(path)?;
    if data.len() < 4 {
        return Err(anyhow!("invalid input: {:?}", path));
    }

    let mut previous: VecDeque<char> = VecDeque::new();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (i, c) in data.chars().enumerate() {
        // Add the new one.
        counts.insert(c, 1 + counts.get(&c).unwrap_or(&0));
        previous.push_back(c);

        // Remove the old one.
        if previous.len() > span {
            let c2 = previous.pop_front().unwrap();
            let n = counts.get(&c2).unwrap();
            if *n == 1 {
                counts.remove(&c2);
            } else {
                counts.insert(c2, n - 1);
            }
        }

        if previous.len() == span && counts.len() == span {
            println!("{}", i + 1);
            return Ok(());
        }
    }

    Err(anyhow!("no answer found!"))
}

fn main() {
    let args = Args::parse();
    match process(&args.path, if args.part2 { 14 } else { 4 }) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
