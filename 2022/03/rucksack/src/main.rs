use clap::Parser;
use itertools::fold;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

pub fn score(c: char) -> u32 {
    return match c {
        'A'..='Z' => ((c as u32) - ('A' as u32)) + 27,
        'a'..='z' => ((c as u32) - ('a' as u32)) + 1,
        _ => panic!("invalid char: {:?}", c),
    };
}

pub fn process(path: &str) -> Result<(), Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error::InvalidArgument(format!(
                "unable to open file {:?}: {:?}",
                path, error
            )))
        }
    };

    let mut r = BufReader::new(file);
    let mut sum = 0;
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let trimmed = line.trim();

        if trimmed == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        let split_point = trimmed.len() / 2;
        let (part1, part2) = trimmed.split_at(split_point);
        let set1 = part1.chars().collect::<HashSet<char>>();
        let set2 = part2.chars().collect::<HashSet<char>>();
        let intersection = set1.intersection(&set2).copied().collect::<Vec<char>>();
        let s = fold(intersection, 0, |n, c| n + score(c));
        sum = sum + s;
    }

    println!("{}", sum);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
