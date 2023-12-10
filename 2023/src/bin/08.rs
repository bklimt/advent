use anyhow::{anyhow, Context, Result};
use clap::Parser;
use num::integer::lcm;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Debug)]
enum Direction {
    Left = 0,
    Right,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

impl Input {
    fn read(path: &str, _debug: bool) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
        let mut r = BufReader::new(file);
        let mut directions: Option<String> = None;
        let mut map = HashMap::new();
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

            if directions.is_none() {
                directions = Some(line.to_owned());
            } else {
                // AAA = (BBB, CCC)
                if line.len() != 16 {
                    return Err(anyhow!("invalid map line: {}", line));
                }
                let start = line[..3].to_owned();
                let left = line[7..10].to_owned();
                let right = line[12..15].to_owned();
                map.insert(start, (left, right));
            }
        }
        let directions = directions.context("missing directions")?;
        let directions: Result<Vec<Direction>> = directions
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(anyhow!("invalid direction: {}", c)),
            })
            .collect();
        let directions = directions?;

        Ok(Input { directions, map })
    }

    fn part1(&self) -> Result<u64> {
        let mut total = 0;
        let mut i = 0;
        let mut current: String = "AAA".to_owned();
        while current != "ZZZ" {
            let (left, right) = self
                .map
                .get(&current)
                .context(format!("unknown position: {}", current))?;

            current = match self.directions[i] {
                Direction::Left => left.clone(),
                Direction::Right => right.clone(),
            };

            total += 1;
            i = (i + 1) % self.directions.len();
        }
        Ok(total)
    }

    fn find_cycle(&self, start: String, debug: bool) -> Result<u64> {
        // Map of (direction index, location) -> steps taken.
        let mut seen: HashMap<(usize, String), u64> = HashMap::new();
        // The step and name of any destination we've seen.
        let mut z: Option<(u64, String)> = None;
        let mut step: u64 = 0;
        let mut current: String = start;
        loop {
            let i: usize = (step as usize) % self.directions.len();

            if let Some(prev) = seen.get(&(i, current.clone())) {
                if let Some((dest_step, dest)) = z {
                    if debug {
                        println!(
                            "loop from {} to {} with dest {} at {}",
                            step, *prev, dest, dest_step
                        );
                    }
                    if step - dest_step != *prev {
                        return Err(anyhow!("cycle would have non-zero offset"));
                    }
                    return Ok(dest_step);
                } else {
                    return Err(anyhow!("loop without a dest!"));
                }
            }
            seen.insert((i, current.clone()), step);

            if current.ends_with("Z") {
                match z {
                    Some((s, n)) => {
                        return Err(anyhow!(
                            "at {}, {}, but already saw {}, {}",
                            step,
                            current,
                            s,
                            n
                        ));
                    }
                    None => {
                        z = Some((step, current.clone()));
                    }
                }
            }

            let (left, right) = self
                .map
                .get(&current)
                .context(format!("unknown position: {}", current))?;

            current = match self.directions[i] {
                Direction::Left => left.clone(),
                Direction::Right => right.clone(),
            };

            step += 1;
        }
    }

    fn part2(&self, debug: bool) -> Result<u64> {
        let mut ans: u64 = 1;
        for node in self.map.keys() {
            if node.ends_with("A") {
                let cycle = self.find_cycle(node.clone(), debug)?;
                ans = lcm(ans, cycle);
            }
        }
        Ok(ans)
    }
}

fn process(args: &Args) -> Result<()> {
    let input = Input::read(&args.input, args.debug)?;
    println!("ans1: {}", input.part1()?);
    println!("ans2: {}", input.part2(args.debug)?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
