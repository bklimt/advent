use advent::common::read_grid;
use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

struct Plot([bool; 65]);

fn process(args: &Args) -> Result<()> {
    let mut grid = read_grid(&args.input, |c| match c {
        '.' | 'S' => Ok(Some(Plot([false; 65]))),
        '#' => Ok(None),
        _ => Err(anyhow!("invalid char: {}", c)),
    })?;

    {
        let start = grid[(65, 65)].as_mut().context("start must be a plot")?;
        start.0[0] = true;
    }

    for i in 1..=64 {
        for r in 0..grid.rows() {
            for c in 0..grid.columns() {
                if grid[(r, c)].as_ref().is_none() {
                    continue;
                }
                let mut reachable = false;
                if r > 0 {
                    if let Some(other) = grid[(r - 1, c)].as_ref() {
                        if other.0[i - 1] {
                            reachable = true;
                        }
                    }
                }
                if c > 0 {
                    if let Some(other) = grid[(r, c - 1)].as_ref() {
                        if other.0[i - 1] {
                            reachable = true;
                        }
                    }
                }
                if r < grid.rows() - 1 {
                    if let Some(other) = grid[(r + 1, c)].as_ref() {
                        if other.0[i - 1] {
                            reachable = true;
                        }
                    }
                }
                if c < grid.columns() - 1 {
                    if let Some(other) = grid[(r, c + 1)].as_ref() {
                        if other.0[i - 1] {
                            reachable = true;
                        }
                    }
                }
                if reachable {
                    if let Some(plot) = grid[(r, c)].as_mut() {
                        plot.0[i] = true;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.columns() {
            if let Some(plot) = grid[(r, c)].as_ref() {
                if plot.0[64] {
                    total += 1;
                }
            }
        }
    }
    println!("ans1 = {}", total);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
