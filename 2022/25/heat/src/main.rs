use anyhow::{anyhow, Context, Result};
use clap::Parser;
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

fn decode(s: &str) -> Result<i64> {
    let mut place = 1i64;
    let mut pos = 0i64;
    let mut neg = 0i64;
    for c in s.chars().rev() {
        match c {
            '2' => {
                pos = pos + 2 * place;
            }
            '1' => {
                pos = pos + place;
            }
            '0' => {}
            '-' => {
                neg = neg + place;
            }
            '=' => {
                neg = neg + 2 * place;
            }
            _ => {
                return Err(anyhow!("invalid character: {}", c));
            }
        }
        place = place * 5;
    }
    Ok(pos - neg)
}

fn read_input(path: &str, debug: bool) -> Result<Vec<i64>> {
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

        let n = decode(line)?;
        if debug {
            println!("{} => {}", line, n);
        }
        v.push(n);
    }
    Ok(v)
}

fn process(args: &Args) -> Result<()> {
    let _ = read_input(&args.input, args.debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
