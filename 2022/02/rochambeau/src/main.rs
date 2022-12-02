use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
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

fn beats(you: char, other: char) -> Result<u32, Error> {
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

fn score(matches: &Vec<(char, char)>, xlt: HashMap<char, char>) -> Result<u32, Error> {
    let mut score = 0;
    for (other, you_secret) in matches.iter() {
        let you = xlt.get(you_secret).unwrap();
        // A: Rock
        // B: Paper
        // C: Scissors

        let shape_score = match you {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => return Err(Error::InvalidArgument(format!("invalid play {:?}", you))),
        };

        let winner_score = beats(*you, *other)?;
        let round_score = winner_score + shape_score;
        println!("round: {:?}", round_score);
        score = score + round_score;
    }
    Ok(score)
}

fn decode(s: &str) -> HashMap<char, char> {
    let mut m: HashMap<char, char> = HashMap::new();
    m.insert('X', s.chars().nth(0).unwrap());
    m.insert('Y', s.chars().nth(1).unwrap());
    m.insert('Z', s.chars().nth(2).unwrap());
    m
}

fn compute(path: &str) -> Result<(), Error> {
    let matches = load(path)?;
    let mut mappings: Vec<String> = Vec::new();

    mappings.push("ABC".to_string());
    /*
    mappings.push("ACB".to_string());
    mappings.push("BAC".to_string());
    mappings.push("BCA".to_string());
    mappings.push("CAB".to_string());
    mappings.push("CBA".to_string());
    */

    for mapping in mappings.iter() {
        let m = decode(mapping);
        let s = score(&matches, m)?;
        println!("{:?}: {:?}", mapping, s);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    match compute(&args.path) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
