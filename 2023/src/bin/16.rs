use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    NORTH = 1,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    dir: Direction,
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Input {
    fn fill(&mut self, x: usize, y: usize, dir: Direction) -> usize {
        let width = self.width;
        let height = self.height;

        let mut q: VecDeque<Beam> = VecDeque::new();
        let mut seen: HashSet<Beam> = HashSet::new();
        let mut on: Vec<Vec<bool>> = vec![vec![false; width]; height];

        let mut score = 0usize;
        q.push_back(Beam { x, y, dir });

        while let Some(beam) = q.pop_front() {
            let &c = self
                .map
                .get(beam.y)
                .expect("y in range")
                .get(beam.x)
                .expect("y in range");

            // If we hit anything other than '.', check whether we've been here before.
            // Repeats are rare, and dots are common, so only check for repeats on other obstacles.
            if c != '.' {
                if !seen.insert(beam.clone()) {
                    continue;
                }
            }

            let is_on: &mut bool = on
                .get_mut(beam.y)
                .expect("y in range")
                .get_mut(beam.x)
                .expect("y in range");
            if !*is_on {
                *is_on = true;
                score += 1;
            }

            let dir = beam.dir;

            match dir {
                Direction::NORTH => {
                    if c == '.' || c == '|' {
                        if beam.y > 0 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y - 1,
                                dir,
                            });
                        }
                    }
                    if c == '/' || c == '-' {
                        if beam.x < width - 1 {
                            q.push_back(Beam {
                                x: beam.x + 1,
                                y: beam.y,
                                dir: Direction::EAST,
                            });
                        }
                    }
                    if c == '\\' || c == '-' {
                        if beam.x > 0 {
                            q.push_back(Beam {
                                x: beam.x - 1,
                                y: beam.y,
                                dir: Direction::WEST,
                            });
                        }
                    }
                }
                Direction::SOUTH => {
                    if c == '.' || c == '|' {
                        if beam.y < height - 1 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y + 1,
                                dir,
                            });
                        }
                    }
                    if c == '/' || c == '-' {
                        if beam.x > 0 {
                            q.push_back(Beam {
                                x: beam.x - 1,
                                y: beam.y,
                                dir: Direction::WEST,
                            });
                        }
                    }
                    if c == '\\' || c == '-' {
                        if beam.x < width - 1 {
                            q.push_back(Beam {
                                x: beam.x + 1,
                                y: beam.y,
                                dir: Direction::EAST,
                            });
                        }
                    }
                }
                Direction::WEST => {
                    if c == '.' || c == '-' {
                        if beam.x > 0 {
                            q.push_back(Beam {
                                x: beam.x - 1,
                                y: beam.y,
                                dir,
                            });
                        }
                    }
                    if c == '/' || c == '|' {
                        if beam.y < height - 1 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y + 1,
                                dir: Direction::SOUTH,
                            });
                        }
                    }
                    if c == '\\' || c == '|' {
                        if beam.y > 0 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y - 1,
                                dir: Direction::NORTH,
                            });
                        }
                    }
                }
                Direction::EAST => {
                    if c == '.' || c == '-' {
                        if beam.x < width - 1 {
                            q.push_back(Beam {
                                x: beam.x + 1,
                                y: beam.y,
                                dir,
                            });
                        }
                    }
                    if c == '/' || c == '|' {
                        if beam.y > 0 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y - 1,
                                dir: Direction::NORTH,
                            });
                        }
                    }
                    if c == '\\' || c == '|' {
                        if beam.y < height - 1 {
                            q.push_back(Beam {
                                x: beam.x,
                                y: beam.y + 1,
                                dir: Direction::SOUTH,
                            });
                        }
                    }
                }
            }
        }
        score
    }
}

fn read_input(path: &str, _debug: bool) -> Result<Input> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);

    let mut v = Vec::new();

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

        v.push(line.chars().collect_vec());
    }

    let height = v.len();
    let width = v.first().expect("map not empty").len();

    Ok(Input {
        map: v,
        width,
        height,
    })
}

fn process(args: &Args) -> Result<()> {
    let mut input = read_input(&args.input, args.debug)?;
    let score = input.fill(0, 0, Direction::EAST);
    println!("ans 1: {}", score);

    let mut best = 0;
    for i in 0..input.width {
        best = best.max(input.fill(i, 0, Direction::SOUTH));
        best = best.max(input.fill(i, input.height - 1, Direction::NORTH));
    }
    for i in 0..input.height {
        best = best.max(input.fill(0, i, Direction::EAST));
        best = best.max(input.fill(input.width - 1, i, Direction::NORTH));
    }
    println!("ans 2: {}", best);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
