use anyhow::{anyhow, Context, Result};
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

pub fn read_data(path: &str) -> Result<String> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut line = String::new();
    r.read_line(&mut line).unwrap();
    let line = line.trim();
    Ok(line.to_string())
}

pub fn process(path: &str, _part2: bool) -> Result<()> {
    let data = read_data(path)?;
    if data.len() < 4 {
        return Err(anyhow!("invalid input: {:?}", path));
    }

    let mut a = data.chars().nth(0).unwrap();
    let mut b = data.chars().nth(1).unwrap();
    let mut c = data.chars().nth(2).unwrap();
    let mut d = data.chars().nth(3).unwrap();

    let mut i: usize = 4;
    loop {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            println!("{}", &data[i - 4..i]);
            println!("{}", i);
            return Ok(());
        }

        if i >= data.len() {
            break;
        }

        a = b;
        b = c;
        c = d;
        d = data.chars().nth(i).unwrap();
        i = i + 1;
    }

    Err(anyhow!("no answer found!"))
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
