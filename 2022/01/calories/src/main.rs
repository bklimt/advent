use clap::Parser;
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

pub fn load(path: &str) -> Result<(), Error> {
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
    let mut sum: i64 = 0;
    let mut max: Vec<i64> = Vec::new();
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let trimmed = line.trim();

        if trimmed == "" {
            // println!("Sum: {:?}", sum);
            max.push(sum);
            max.sort_by(|a, b| b.cmp(a));
            if max.len() > 3 {
                max.pop();
            }

            sum = 0;
            if n == 0 {
                break;
            }
            continue;
        }

        let amount = match trimmed.parse::<i64>() {
            Ok(f) => f,
            Err(error) => {
                return Err(Error::InvalidArgument(format!(
                    "invalid number {:?}: {:?}",
                    trimmed, error
                )))
            }
        };

        sum = sum + amount;
        // println!("amount: {:?}", amount);
    }

    let mut top3: i64 = 0;
    for n in max.iter() {
        top3 = top3 + n;
    }

    println!("Max: {:?}", max[0]);
    println!("Top 3: {:?}", top3);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match load(&args.path) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
