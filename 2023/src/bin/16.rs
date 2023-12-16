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

#[derive(Debug)]
struct Node {
    c: char,
    on: bool,
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
    map: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Input {
    fn fill(&mut self, x: usize, y: usize, dir: Direction) {
        let width = self.width;
        let height = self.height;

        let mut q: VecDeque<Beam> = VecDeque::new();
        let mut seen: HashSet<Beam> = HashSet::new();

        q.push_back(Beam { x, y, dir });

        while let Some(beam) = q.pop_front() {
            if !seen.insert(beam.clone()) {
                continue;
            }

            let node: &mut Node = self
                .map
                .get_mut(beam.y)
                .expect("y in range")
                .get_mut(beam.x)
                .expect("y in range");
            node.on = true;

            let dir = beam.dir;
            let c = node.c;

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
    }

    fn score_and_clear(&mut self) -> usize {
        let mut score = 0;
        for (_, row) in self.map.iter_mut().enumerate() {
            for node in row.iter_mut() {
                if node.on {
                    score += 1;
                }
                node.on = false;
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

        v.push(line.chars().map(|c| Node { c, on: false }).collect_vec());
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
    input.fill(0, 0, Direction::EAST);
    println!("ans 1: {}", input.score_and_clear());

    let mut best = 0;
    for i in 0..input.width {
        input.fill(i, 0, Direction::SOUTH);
        best = best.max(input.score_and_clear());

        input.fill(i, input.height - 1, Direction::NORTH);
        best = best.max(input.score_and_clear());
    }
    for i in 0..input.height {
        input.fill(0, i, Direction::EAST);
        best = best.max(input.score_and_clear());

        input.fill(input.width - 1, i, Direction::NORTH);
        best = best.max(input.score_and_clear());
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
