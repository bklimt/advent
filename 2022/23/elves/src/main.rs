use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

struct Elf {
    pos: (i64, i64),
    next: (i64, i64),
    active: bool,
}

#[derive(Debug)]
struct Bounds {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

struct Map {
    elves: Vec<Elf>,
    bounds: Bounds,
    dir: Direction,
}

impl Map {
    fn step(&mut self) {
        // Gather all of the start positions.
        let mut start_pos = HashSet::new();
        for elf in self.elves.iter() {
            start_pos.insert(elf.pos);
        }

        for elf in self.elves.iter_mut() {
            elf.active = false;
            let (x, y) = elf.pos;
            elf.active = elf.active || start_pos.contains(&(x - 1, y - 1));
            elf.active = elf.active || start_pos.contains(&(x, y - 1));
            elf.active = elf.active || start_pos.contains(&(x + 1, y - 1));
            elf.active = elf.active || start_pos.contains(&(x - 1, y));
            elf.active = elf.active || start_pos.contains(&(x + 1, y));
            elf.active = elf.active || start_pos.contains(&(x - 1, y + 1));
            elf.active = elf.active || start_pos.contains(&(x, y + 1));
            elf.active = elf.active || start_pos.contains(&(x + 1, y + 1));
        }

        // Find all the proposals for next spots.
        let mut proposals = HashSet::new();
        let mut conflicts = HashSet::new();
        for elf in self.elves.iter_mut() {
            if !elf.active {
                continue;
            }
            let (x, y) = elf.pos;
            elf.next = elf.pos;
            let mut dir = self.dir.clone();
            for _ in 0..4 {
                let mut possible = true;
                let spots = match dir {
                    Direction::North => vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)],
                    Direction::South => vec![(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
                    Direction::East => vec![(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)],
                    Direction::West => vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)],
                };
                for spot in spots {
                    if start_pos.contains(&spot) {
                        possible = false;
                        break;
                    }
                }
                if possible {
                    elf.next = match dir {
                        Direction::North => (x, y - 1),
                        Direction::South => (x, y + 1),
                        Direction::East => (x + 1, y),
                        Direction::West => (x - 1, y),
                    };
                    if proposals.contains(&elf.next) {
                        conflicts.insert(elf.next);
                    } else {
                        proposals.insert(elf.next);
                    }
                    break;
                }
                dir = dir.next();
            }
        }

        // Cancel out all the conflicts.
        for elf in self.elves.iter_mut() {
            if !elf.active {
                continue;
            }
            if conflicts.contains(&elf.next) {
                elf.next = elf.pos;
            }
        }

        // Commit the moves.
        let mut bounds = Bounds {
            min_x: i64::MAX,
            min_y: i64::MAX,
            max_x: i64::MIN,
            max_y: i64::MIN,
        };
        for elf in self.elves.iter_mut() {
            if elf.active {
                elf.pos = elf.next;
            }
            bounds.min_x = bounds.min_x.min(elf.pos.0);
            bounds.min_y = bounds.min_y.min(elf.pos.1);
            bounds.max_x = bounds.max_x.max(elf.pos.0);
            bounds.max_y = bounds.max_y.max(elf.pos.1);
        }
        self.bounds = bounds;
        self.dir = self.dir.next();
    }

    fn print(&self) {
        let mut elves = HashMap::new();
        for elf in self.elves.iter() {
            elves.insert(elf.pos, '#');
        }
        for y in self.bounds.min_y..=self.bounds.max_y {
            for x in self.bounds.min_x..=self.bounds.max_x {
                print!("{}", elves.get(&(x, y)).unwrap_or(&'.'));
            }
            println!("");
        }
    }

    fn score(&self) -> i64 {
        let width = (self.bounds.max_x - self.bounds.min_x) + 1;
        let height = (self.bounds.max_y - self.bounds.min_y) + 1;
        let area = width * height;
        area - (self.elves.len() as i64)
    }
}

fn read_input(path: &str, debug: bool) -> Result<Map> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut map = Map {
        elves: Vec::new(),
        bounds: Bounds {
            min_x: i64::MAX,
            min_y: i64::MAX,
            max_x: i64::MIN,
            max_y: i64::MIN,
        },
        dir: Direction::North,
    };
    let mut y = 0;
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        if debug {
            println!("line: {}", line);
        }
        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => {}
                '#' => {
                    let elf = Elf {
                        pos: (x, y),
                        next: (0, 0),
                        active: true,
                    };
                    map.bounds.min_x = map.bounds.min_x.min(x);
                    map.bounds.min_y = map.bounds.min_y.min(y);
                    map.bounds.max_x = map.bounds.max_x.max(x);
                    map.bounds.max_y = map.bounds.max_y.max(y);
                    map.elves.push(elf);
                }
                _ => return Err(anyhow!("unknown char: {}", c)),
            }
            x = x + 1;
        }
        y = y + 1;
    }
    Ok(map)
}

fn process(args: &Args) -> Result<()> {
    let mut map = read_input(&args.input, args.debug)?;
    if args.debug {
        println!("");
        println!("== Initial State ==");
        map.print();
        println!("");
    }
    for i in 1..=10 {
        map.step();
        if args.debug {
            println!("== End of Round {} ==", i);
            map.print();
            println!("bounds: {:?}", map.bounds);
            println!("");
        }
    }
    println!("ans = {}", map.score());
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
