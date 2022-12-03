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

    #[arg(long)]
    part2: bool,
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

pub fn process1(path: &str) -> Result<(), Error> {
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

pub fn process2(path: &str) -> Result<(), Error> {
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
    let mut triad: Vec<HashSet<char>> = Vec::new();
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

        triad.push(trimmed.chars().collect());
        if triad.len() == 3 {
            let one = triad.pop().unwrap();
            let two = triad.pop().unwrap();
            let three = triad.pop().unwrap();
            let common = one
                .intersection(&two)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&three)
                .copied()
                .collect::<Vec<char>>();

            let s = fold(common, 0, |n, c| n + score(c));
            sum = sum + s;
            triad.clear();
        }
    }

    println!("{}", sum);

    Ok(())
}

pub fn process(path: &str, part2: bool) -> Result<(), Error> {
    if part2 {
        process2(path)
    } else {
        process1(path)
    }
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
