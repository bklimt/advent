use advent::common::{read_lines, StrIterator};
use anyhow::{anyhow, Result};
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

fn next_value(v: &Vec<i64>, part2: bool, debug: bool) -> Result<i64> {
    if debug {
        println!("{:?}", v);
    }
    if v.len() == 0 {
        return Err(anyhow!("empty vec"));
    }

    let mut next = Vec::new();
    let mut all_zeroes = true;
    for (i, n) in v.iter().enumerate() {
        if *n != 0 {
            all_zeroes = false;
        }
        if i > 0 {
            next.push(n - v[i - 1]);
        }
    }
    if all_zeroes {
        return Ok(0);
    }

    let d = next_value(&next, part2, debug)?;
    if debug {
        println!("delta = {}", d);
    }
    Ok(if part2 {
        v.first().expect("vec should not be empty") - d
    } else {
        v.last().expect("vec should not be empty") + d
    })
}

fn extrapolate(path: &str, part2: bool, debug: bool) -> Result<i64> {
    let mut total = 0;
    for line in read_lines(path)? {
        let nums = line.split_whitespace().parse_all()?;
        total += next_value(&nums, part2, debug)?;
    }
    Ok(total)
}

fn process(args: &Args) -> Result<()> {
    println!("ans1: {}", extrapolate(&args.input, false, args.debug)?);
    println!("ans2: {}", extrapolate(&args.input, true, args.debug)?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
