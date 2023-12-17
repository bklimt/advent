use advent::common::{read_grid, Array2D};
use anyhow::{Context, Error, Result};
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
    Ok(read_grid::<i32, Error>(args.input.as_str(), |c| {
        Ok(c.to_digit(10).context("invalid digit")? as i32)
    })?)
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
