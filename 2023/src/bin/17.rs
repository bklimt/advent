use advent::common::{read_lines, Array2D};
use anyhow::Result;
use clap::Parser;
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn process(args: &Args) -> Result<Array2D<i32>> {
    let mut v = Vec::new();
    for line in read_lines(args.input.as_str())? {
        let mut row = Vec::new();
        for c in line.chars() {
            let d = c.to_digit(10)?;
            row.push(d as i32);
        }
        v.push(row);
    }
    Ok(v.try_into()?)
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
