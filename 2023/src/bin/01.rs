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

    #[arg(long)]
    part2: bool,
}

fn read_input(path: &str, _debug: bool) -> Result<Vec<String>> {
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

        v.push(line.into());
    }
    Ok(v)
}

fn read_number_part1(line: &str, _debug: bool) -> Result<u32> {
    let p1 = line
        .find(|c: char| c.is_digit(10))
        .context(format!("no digit in line {:?}", line))?;
    let p2 = line
        .rfind(|c: char| c.is_digit(10))
        .context(format!("no digit in line {:?}", line))?;

    let b = line.as_bytes();
    let c1 = b[p1] as char;
    let c2 = b[p2] as char;

    let n1 = c1.to_digit(10).context(format!("invalid digit {:?}", c1))?;
    let n2 = c2.to_digit(10).context(format!("invalid digit {:?}", c2))?;

    Ok(n1 * 10 + n2)
}

fn read_number_part2(line: &str, _debug: bool) -> Result<u32> {
    let mut n1: Option<u32> = None;
    let mut n2: Option<u32> = None;

    let mut s: &str = line;
    while s.len() > 0 {
        let n = if s.chars().nth(0).unwrap().is_digit(10) {
            Some(s.chars().nth(0).unwrap().to_digit(10).unwrap())
        } else if s.starts_with("one") {
            Some(1)
        } else if s.starts_with("two") {
            Some(2)
        } else if s.starts_with("three") {
            Some(3)
        } else if s.starts_with("four") {
            Some(4)
        } else if s.starts_with("five") {
            Some(5)
        } else if s.starts_with("six") {
            Some(6)
        } else if s.starts_with("seven") {
            Some(7)
        } else if s.starts_with("eight") {
            Some(8)
        } else if s.starts_with("nine") {
            Some(9)
        } else {
            None
        };
        s = &s[1..];

        if let Some(n) = n {
            if n1.is_none() {
                n1 = Some(n);
            }
            n2 = Some(n)
        }
    }

    if n1.is_none() {
        return Err(anyhow!("no number in {:?}", line));
    }

    Ok(n1.unwrap() * 10 + n2.unwrap())
}

fn process(args: &Args) -> Result<()> {
    let lines = read_input(&args.input, args.debug)?;
    let mut total = 0;
    for line in lines {
        let n = if args.part2 {
            read_number_part2(line.as_str(), args.debug)?
        } else {
            read_number_part1(line.as_str(), args.debug)?
        };
        println!("{} -> {}", line, n);
        total += n;
    }
    println!("total: {}", total);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
