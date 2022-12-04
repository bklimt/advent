use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

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

pub fn parse_number(s: &str) -> Result<u32, Error> {
    match s.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(error) => Err(Error::InvalidArgument(format!(
            "invalid number {:?}: {:?}",
            s, error
        ))),
    }
}

pub fn parse_range(s: &str) -> Result<RangeInclusive<u32>, Error> {
    let dash = match s.find('-') {
        Some(i) => i,
        None => return Err(Error::InvalidArgument(format!("no dash: {:?}", s))),
    };
    let (ns1, ns2) = s.split_at(dash);
    let ns2 = ns2.strip_prefix('-').unwrap();

    let n1 = parse_number(ns1)?;
    let n2 = parse_number(ns2)?;

    Ok(n1..=n2)
}

pub fn read(path: &str) -> Result<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>, Error> {
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
    let mut result: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = Vec::new();
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

        let comma = match trimmed.find(',') {
            Some(i) => i,
            None => return Err(Error::InvalidArgument(format!("no comma: {:?}", trimmed))),
        };

        let (rs1, rs2) = trimmed.split_at(comma);
        let rs2 = rs2.strip_prefix(',').unwrap();

        let r1 = parse_range(rs1)?;
        let r2 = parse_range(rs2)?;

        result.push(
            if r1.start() < r2.start() || (r1.start() == r2.start() && r1.end() >= r2.end()) {
                (r1, r2)
            } else {
                (r2, r1)
            },
        );
    }

    Ok(result)
}

pub fn is_redundant_pair(pair: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    pair.1.start() <= pair.0.end() && pair.1.end() <= pair.0.end()
}

pub fn process(path: &str, _part2: bool) -> Result<(), Error> {
    let pairs = read(path)?;
    for pair in pairs.iter() {
        println!("{:?}", pair);
    }

    println!("\nRedundant:");

    let redundant = pairs
        .into_iter()
        .filter(is_redundant_pair)
        .collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>();
    for pair in redundant.iter() {
        println!("{:?}", pair);
    }

    println!("\nCount: {:?}", redundant.len());

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
