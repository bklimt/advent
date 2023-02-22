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

    #[arg(long)]
    debug: bool,
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl Entry {
    fn parse(s: &str) -> Result<Entry> {
        let err = || anyhow!("invalid line: {}", s);
        let s = s.strip_prefix("Sensor at x=").ok_or_else(err)?;
        let comma = s.find(',').ok_or_else(err)?;
        let (ssx, s) = s.split_at(comma);
        let s = s.strip_prefix(", y=").ok_or_else(err)?;
        let colon = s.find(':').ok_or_else(err)?;
        let (ssy, s) = s.split_at(colon);
        let s = s
            .strip_prefix(": closest beacon is at x=")
            .ok_or_else(err)?;
        let comma = s.find(',').ok_or_else(err)?;
        let (sbx, s) = s.split_at(comma);
        let sby = s.strip_prefix(", y=").ok_or_else(err)?;

        let sx = ssx.parse::<i64>()?;
        let sy = ssy.parse::<i64>()?;
        let bx = sbx.parse::<i64>()?;
        let by = sby.parse::<i64>()?;

        Ok(Entry {
            sensor: (sx, sy),
            beacon: (bx, by),
        })
    }
}

fn read_input(path: &str, debug: bool) -> Result<Vec<Entry>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut entries = Vec::new();
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

        if debug {
            println!("line: {}", line);
        }
        let entry = Entry::parse(line)?;
        if debug {
            println!("entry: {:?}", entry);
        }
        entries.push(entry);
    }
    Ok(entries)
}

fn process(path: &str, _part2: bool, debug: bool) -> Result<()> {
    let mut _entries = read_input(path, debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2, args.debug) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
