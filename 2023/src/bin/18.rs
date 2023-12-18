use advent::common::{read_lines, split_on, StrIterator};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up = 1,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            bail!("invalid direction: {}", s);
        }
        match s.chars().nth(0) {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Record {
    dir: Direction,
    amount: i64,
    color: String,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (dir, rest) = split_on(s, ' ').context("missing first space")?;
        let (amount, color) = split_on(rest, ' ').context("missing second space")?;

        let dir = dir.parse()?;
        let amount = amount.parse()?;
        let color = color.to_owned();

        Ok(Record { dir, amount, color })
    }
}

fn read_input(path: &str) -> Result<Vec<Record>> {
    Ok(read_lines(path)?.parse_all()?)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(args.input.as_str())?;
    let mut row = 0i64;
    let mut col = 0i64;
    let mut min_row = 0i64;
    let mut max_row = 0i64;
    let mut min_col = 0i64;
    let mut max_col = 0i64;
    for rec in input.iter() {
        match rec.dir {
            Direction::Up => row -= rec.amount,
            Direction::Down => row += rec.amount,
            Direction::Left => col -= rec.amount,
            Direction::Right => col += rec.amount,
        }
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }
    println!(
        "Position ranges from ({}, {}) to ({}, {})",
        min_row, min_col, max_row, max_col
    );
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
