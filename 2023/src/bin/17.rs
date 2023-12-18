use advent::common::{read_grid, Array2D};
use anyhow::{bail, Context, Error, Result};
use clap::Parser;
use priority_queue::DoublePriorityQueue;
use std::{collections::HashMap, option::Option};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

struct Input {
    grid: Array2D<i32>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North = 1,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    row: usize,
    col: usize,
    dir: Direction,
    count: u8,
}

impl Node {
    fn plus_dir(&self, dir: Direction) -> Node {
        let count = if dir == self.dir { self.count + 1 } else { 1 };
        let (dr, dc): (i32, i32) = match dir {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        };
        Node {
            row: ((self.row as i32) + dr) as usize,
            col: ((self.col as i32) + dc) as usize,
            dir,
            count,
        }
    }
}

impl Input {
    fn read(path: &str) -> Result<Self> {
        let grid =
            read_grid::<i32, Error>(
                path,
                |c| Ok(c.to_digit(10).context("invalid digit")? as i32),
            )?;
        Ok(Input { grid })
    }

    fn can_move(&self, node: &Node, dir: Direction) -> bool {
        if dir == node.dir && node.count >= 3 {
            return false;
        }
        match dir {
            Direction::North => node.dir != Direction::South && node.row > 0,
            Direction::South => node.dir != Direction::North && node.row < self.grid.rows() - 1,
            Direction::West => node.dir != Direction::East && node.col > 0,
            Direction::East => node.dir != Direction::West && node.col < self.grid.columns() - 1,
        }
    }

    fn try_move(&self, node: &Node, dir: Direction) -> Option<Node> {
        if self.can_move(node, dir) {
            Some(node.plus_dir(dir))
        } else {
            None
        }
    }

    fn next(&self, node: &Node) -> Vec<Node> {
        let mut v = Vec::new();
        if let Some(next) = self.try_move(node, Direction::North) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::South) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::East) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::West) {
            v.push(next);
        }
        v
    }

    fn search(&self, debug: bool) -> Result<i64> {
        let start = Node {
            row: 0,
            col: 0,
            dir: Direction::South,
            count: 0,
        };
        let mut dist: HashMap<Node, i64> = HashMap::new();
        dist.insert(start, 0);

        let mut q = DoublePriorityQueue::new();
        q.push(start, 0);
        while let Some((current, d)) = q.pop_min() {
            if debug {
                println!("visiting {:?} = {}", current, d);
            }

            if current.row == self.grid.rows() - 1 && current.col == self.grid.columns() - 1 {
                return Ok(d);
            }

            let next = self.next(&current);
            for n in next {
                let d2 = d + self.grid[(n.row, n.col)] as i64;
                if debug {
                    println!("neighbor {:?} = {}", n, d2);
                }
                let best = if let Some(&prev) = dist.get(&n) {
                    d2 < prev
                } else {
                    true
                };
                if best {
                    if debug {
                        println!("updating to {}", d2);
                    }
                    dist.insert(n.clone(), d2);
                    q.push_decrease(n, d2);
                }
            }
        }

        bail!("no result found");
    }
}

fn process(args: &Args) -> Result<()> {
    let input = Input::read(args.input.as_str())?;
    let ans = input.search(args.debug)?;
    println!("ans = {}", ans);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
