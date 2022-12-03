use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

pub fn load(path: &str) -> Result<Vec<(char, char)>, Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            return Err(Error::InvalidArgument(format!(
                "unable to open file {:?}: {:?}",
                path, error
            )))
        }
    };

    let mut r = BufReader::new(file);
    let mut matches: Vec<(char, char)> = Vec::new();
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let trimmed = line.trim();

        if trimmed == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        if trimmed.len() != 3 || trimmed.bytes().nth(1).unwrap() != 32 {
            return Err(Error::InvalidArgument(format!(
                "invalid line {:?}",
                trimmed
            )));
        }

        let (p1, p2) = trimmed.split_at(1);
        matches.push((p1.chars().nth(0).unwrap(), p2.chars().nth(1).unwrap()));
    }

    Ok(matches)
}

fn beats(you: &char, other: &char) -> Result<u32, Error> {
    // A: Rock
    // B: Paper
    // C: Scissors
    let score = match you {
        'A' => match other {
            'A' => 3,
            'B' => 0,
            'C' => 6,
            _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", other))),
        },
        'B' => match other {
            'A' => 6,
            'B' => 3,
            'C' => 0,
            _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", other))),
        },
        'C' => match other {
            'A' => 0,
            'B' => 6,
            'C' => 3,
            _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", other))),
        },
        _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", you))),
    };
    Ok(score)
}

fn part1(you_secret: &char, _other: &char) -> char {
    return match you_secret {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => {
            panic!("bad letter: {:?}", you_secret);
        }
    };
}

fn part2(you_secret: &char, other: &char) -> char {
    return match you_secret {
        'X' => match other {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => {
                panic!("bad letter: {:?}", other);
            }
        },
        'Y' => *other,
        'Z' => match other {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => {
                panic!("bad letter: {:?}", other);
            }
        },
        _ => {
            panic!("bad letter: {:?}", you_secret);
        }
    };
}

fn score(matches: &Vec<(char, char)>, xlt: fn(&char, &char) -> char) -> Result<u32, Error> {
    let mut score = 0;
    for (other, you_secret) in matches.iter() {
        let you = xlt(you_secret, other);
        // A: Rock
        // B: Paper
        // C: Scissors

        let shape_score = match you {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", you))),
        };

        let winner_score = beats(&you, other)?;
        let round_score = winner_score + shape_score;
        // println!("round: {:?}", round_score);
        score = score + round_score;
    }
    Ok(score)
}

fn compute(path: &str, p2: bool) -> Result<(), Error> {
    let matches = load(path)?;

    let s = score(&matches, if p2 { part2 } else { part1 })?;
    println!("{:?}", s);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match compute(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
