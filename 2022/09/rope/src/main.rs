use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,

    #[arg(long)]
    debug: bool,
}

fn process_input(path: &str, len: usize, debug: bool) -> Result<u32> {
    let mut rope: Vec<(i32, i32)> = Vec::new();
    rope.resize(len, (0, 0));

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 5;
    let mut max_y = 5;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope[0]);

    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
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

        let command = line
            .chars()
            .nth(0)
            .ok_or_else(|| anyhow!("invalid command: {}", line))?;

        if line
            .chars()
            .nth(1)
            .ok_or_else(|| anyhow!("invalid command: {}", line))?
            != ' '
        {
            return Err(anyhow!("invalid command: {}", line));
        }

        let (_, amount_str) = line.split_at(2);
        let amount = amount_str
            .parse::<i32>()
            .with_context(|| format!("invalid amount: {:?} in {}", amount_str, line))?;

        if debug {
            println!("\n== {} {} ==", command, amount);
        }

        for _ in 0..amount {
            match command {
                'R' => {
                    rope[0].0 = rope[0].0 + 1;
                }
                'L' => {
                    rope[0].0 = rope[0].0 - 1;
                }
                'D' => {
                    rope[0].1 = rope[0].1 - 1;
                }
                'U' => {
                    rope[0].1 = rope[0].1 + 1;
                }
                _ => {
                    return Err(anyhow!("invalid input: {}", line));
                }
            };

            // visited.insert(rope[0]);
            min_x = min_x.min(rope[0].0);
            min_y = min_y.min(rope[0].1);
            max_x = max_x.max(rope[0].0);
            max_y = max_y.max(rope[0].1);

            for i in 1..rope.len() {
                if rope[i - 1].0 == rope[i].0 {
                    if rope[i].1 < rope[i - 1].1 - 1 {
                        rope[i].1 = rope[i - 1].1 - 1;
                    }
                    if rope[i].1 > rope[i - 1].1 + 1 {
                        rope[i].1 = rope[i - 1].1 + 1;
                    }
                } else if rope[i - 1].1 == rope[i].1 {
                    if rope[i].0 < rope[i - 1].0 - 1 {
                        rope[i].0 = rope[i - 1].0 - 1;
                    }
                    if rope[i].0 > rope[i - 1].0 + 1 {
                        rope[i].0 = rope[i - 1].0 + 1;
                    }
                } else {
                    /*
                     *   IA.CJ
                     *   B...D
                     *   ..H..
                     *   E...G
                     *   KF.HL
                     */
                    if rope[i].0 == rope[i - 1].0 - 1 && rope[i].1 == rope[i - 1].1 - 2 {
                        rope[i].0 = rope[i - 1].0;
                        rope[i].1 = rope[i - 1].1 - 1;
                    } else if rope[i].0 == rope[i - 1].0 - 2 && rope[i].1 == rope[i - 1].1 - 1 {
                        rope[i].0 = rope[i - 1].0 - 1;
                        rope[i].1 = rope[i - 1].1;
                    } else if rope[i].0 == rope[i - 1].0 + 1 && rope[i].1 == rope[i - 1].1 - 2 {
                        rope[i].0 = rope[i - 1].0;
                        rope[i].1 = rope[i - 1].1 - 1;
                    } else if rope[i].0 == rope[i - 1].0 + 2 && rope[i].1 == rope[i - 1].1 - 1 {
                        rope[i].0 = rope[i - 1].0 + 1;
                        rope[i].1 = rope[i - 1].1;
                    } else if rope[i].0 == rope[i - 1].0 - 2 && rope[i].1 == rope[i - 1].1 + 1 {
                        rope[i].0 = rope[i - 1].0 - 1;
                        rope[i].1 = rope[i - 1].1;
                    } else if rope[i].0 == rope[i - 1].0 - 1 && rope[i].1 == rope[i - 1].1 + 2 {
                        rope[i].0 = rope[i - 1].0;
                        rope[i].1 = rope[i - 1].1 + 1;
                    } else if rope[i].0 == rope[i - 1].0 + 2 && rope[i].1 == rope[i - 1].1 + 1 {
                        rope[i].0 = rope[i - 1].0 + 1;
                        rope[i].1 = rope[i - 1].1;
                    } else if rope[i].0 == rope[i - 1].0 + 1 && rope[i].1 == rope[i - 1].1 + 2 {
                        rope[i].0 = rope[i - 1].0;
                        rope[i].1 = rope[i - 1].1 + 1;
                    } else if rope[i].0 == rope[i - 1].0 - 2 && rope[i].1 == rope[i - 1].1 - 2 {
                        rope[i].0 = rope[i - 1].0 - 1;
                        rope[i].1 = rope[i - 1].1 - 1;
                    } else if rope[i].0 == rope[i - 1].0 + 2 && rope[i].1 == rope[i - 1].1 - 2 {
                        rope[i].0 = rope[i - 1].0 + 1;
                        rope[i].1 = rope[i - 1].1 - 1;
                    } else if rope[i].0 == rope[i - 1].0 - 2 && rope[i].1 == rope[i - 1].1 + 2 {
                        rope[i].0 = rope[i - 1].0 - 1;
                        rope[i].1 = rope[i - 1].1 + 1;
                    } else if rope[i].0 == rope[i - 1].0 + 2 && rope[i].1 == rope[i - 1].1 + 2 {
                        rope[i].0 = rope[i - 1].0 + 1;
                        rope[i].1 = rope[i - 1].1 + 1;
                    }
                }
                // visited.insert(rope[i]);
                min_x = min_x.min(rope[i].0);
                min_y = min_y.min(rope[i].1);
                max_x = max_x.max(rope[i].0);
                max_y = max_y.max(rope[i].1);
            }
            visited.insert(rope[rope.len() - 1]);

            if debug {
                for y in (min_y..=max_y).rev() {
                    for x in min_x..=max_x {
                        if rope[0].0 == x && rope[0].1 == y {
                            print!("H");
                            continue;
                        }
                        let mut seen = false;
                        for i in 1..rope.len() {
                            if rope[i].0 == x && rope[i].1 == y {
                                print!("{}", i);
                                seen = true;
                                break;
                            }
                        }
                        if seen {
                            continue;
                        }
                        print!(".");
                    }
                    println!("");
                }
                println!("");
            }
        }
    }
    Ok(visited.len() as u32)
}

fn process(path: &str, part2: bool, debug: bool) -> Result<()> {
    let visited = process_input(path, if part2 { 10 } else { 2 }, debug)?;
    // 6236
    println!("part1 = {}", visited);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2, args.debug) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
