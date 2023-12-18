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

// Node is a location and how far you've traveled so far in the current direction.
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

    fn can_move(&self, node: &Node, dir: Direction, min: u8, max: u8) -> bool {
        if dir != node.dir && node.count < min {
            return false;
        }
        if dir == node.dir && node.count >= max {
            return false;
        }
        match dir {
            Direction::North => node.dir != Direction::South && node.row > 0,
            Direction::South => node.dir != Direction::North && node.row < self.grid.rows() - 1,
            Direction::West => node.dir != Direction::East && node.col > 0,
            Direction::East => node.dir != Direction::West && node.col < self.grid.columns() - 1,
        }
    }

    fn try_move(&self, node: &Node, dir: Direction, min: u8, max: u8) -> Option<Node> {
        if self.can_move(node, dir, min, max) {
            Some(node.plus_dir(dir))
        } else {
            None
        }
    }

    // Returns the set of nodes you can get to from this one.
    fn next(&self, node: &Node, min: u8, max: u8) -> Vec<Node> {
        let mut v = Vec::new();
        if let Some(next) = self.try_move(node, Direction::North, min, max) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::South, min, max) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::East, min, max) {
            v.push(next);
        }
        if let Some(next) = self.try_move(node, Direction::West, min, max) {
            v.push(next);
        }
        v
    }

    fn print_path(prev: &HashMap<Node, Node>, curr: &Node) {
        if let Some(n) = prev.get(curr) {
            Input::print_path(&prev, n);
        }
        println!("{:?}", curr);
    }

    // This is basically Dijkstra's algorithm on the graph of Nodes.
    fn search(&self, part2: bool, debug: bool) -> Result<i64> {
        let (min, max) = if part2 { (4, 10) } else { (0, 3) };

        let start1 = Node {
            row: 0,
            col: 0,
            dir: Direction::South,
            count: 0,
        };
        let start2 = Node {
            row: 0,
            col: 0,
            dir: Direction::East,
            count: 0,
        };
        let mut dist: HashMap<Node, i64> = HashMap::new();
        dist.insert(start1, 0);
        dist.insert(start2, 0);

        let mut prev: HashMap<Node, Node> = HashMap::new();

        let mut q = DoublePriorityQueue::new();
        q.push(start1, 0);
        q.push(start2, 0);
        while let Some((current, d)) = q.pop_min() {
            if debug {
                println!("visiting {:?} = {}", current, d);
            }

            if current.count >= min
                && current.row == self.grid.rows() - 1
                && current.col == self.grid.columns() - 1
            {
                if debug {
                    println!("returning {}", d);
                    Input::print_path(&prev, &current);
                }
                return Ok(d);
            }

            let next = self.next(&current, min, max);
            for n in next {
                let d2 = d + self.grid[(n.row, n.col)] as i64;
                if debug {
                    println!("neighbor {:?} = {}", n, d2);
                }
                let best = if let Some(&d0) = dist.get(&n) {
                    d2 < d0
                } else {
                    true
                };
                if best {
                    if debug {
                        println!("updating to {}", d2);
                    }
                    dist.insert(n.clone(), d2);
                    prev.insert(n.clone(), current.clone());
                    q.push_decrease(n, d2);
                }
            }
        }

        bail!("no result found");
    }
}

fn process(args: &Args) -> Result<()> {
    let input = Input::read(args.input.as_str())?;

    if args.debug {
        println!("part1:");
    }
    let ans1 = input.search(false, args.debug)?;
    println!("ans1 = {}", ans1);

    if args.debug {
        println!("\npart2:");
    }
    let ans2 = input.search(true, args.debug)?;
    println!("ans2 = {}", ans2);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
