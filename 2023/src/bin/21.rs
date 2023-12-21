use advent::common::{read_grid, Array2D};
use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    part2: bool,

    #[arg(long)]
    iterations: i32,
}

struct Plot {
    current: bool,
    previous: bool,
}

fn read_input(path: &str) -> Result<Array2D<Option<Plot>>> {
    Ok(read_grid(path, |c| match c {
        '.' => Ok(Some(Plot {
            current: false,
            previous: false,
        })),
        'S' => Ok(Some(Plot {
            current: true,
            previous: false,
        })),
        '#' => Ok(None),
        _ => Err(anyhow!("invalid char: {}", c)),
    })?)
}

fn is_reachable1(grid: &Array2D<Option<Plot>>, r: usize, c: usize) -> bool {
    if r > 0 {
        if let Some(other) = grid[(r - 1, c)].as_ref() {
            if other.previous {
                return true;
            }
        }
    }
    if c > 0 {
        if let Some(other) = grid[(r, c - 1)].as_ref() {
            if other.previous {
                return true;
            }
        }
    }
    if r < grid.rows() - 1 {
        if let Some(other) = grid[(r + 1, c)].as_ref() {
            if other.previous {
                return true;
            }
        }
    }
    if c < grid.columns() - 1 {
        if let Some(other) = grid[(r, c + 1)].as_ref() {
            if other.previous {
                return true;
            }
        }
    }
    return false;
}

fn is_reachable2(grid: &Array2D<Option<Plot>>, r: usize, c: usize) -> bool {
    let r_n = (r + grid.rows() - 1) % grid.rows();
    let c_w = (c + grid.columns() - 1) % grid.columns();
    let r_s = (r + 1) % grid.rows();
    let c_e = (c + 1) % grid.columns();
    if let Some(other) = grid[(r_n, c)].as_ref() {
        if other.previous {
            return true;
        }
    }
    if let Some(other) = grid[(r, c_w)].as_ref() {
        if other.previous {
            return true;
        }
    }
    if let Some(other) = grid[(r_s, c)].as_ref() {
        if other.previous {
            return true;
        }
    }
    if let Some(other) = grid[(r, c_e)].as_ref() {
        if other.previous {
            return true;
        }
    }
    return false;
}

fn is_reachable(grid: &Array2D<Option<Plot>>, r: usize, c: usize, part2: bool) -> bool {
    if part2 {
        is_reachable2(grid, r, c)
    } else {
        is_reachable1(grid, r, c)
    }
}
fn process(args: &Args) -> Result<()> {
    let mut grid = read_input(&args.input)?;
    for _ in 1..=args.iterations {
        for r in 0..grid.rows() {
            for c in 0..grid.columns() {
                if let Some(plot) = grid[(r, c)].as_mut() {
                    plot.previous = plot.current;
                    plot.current = false;
                }
            }
        }

        for r in 0..grid.rows() {
            for c in 0..grid.columns() {
                if grid[(r, c)].as_ref().is_none() {
                    continue;
                }
                if is_reachable(&grid, r, c, args.part2) {
                    if let Some(plot) = grid[(r, c)].as_mut() {
                        plot.current = true;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.columns() {
            if let Some(plot) = grid[(r, c)].as_ref() {
                if plot.current {
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
