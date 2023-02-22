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

    #[arg(long)]
    y: i64,
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

    fn dist(&self) -> i64 {
        (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()
    }

    fn range_for_y(&self, y: i64) -> Option<(i64, i64)> {
        let dist_to_row = (self.sensor.1 - y).abs();
        let remaining = self.dist() - dist_to_row;
        if remaining < 0 {
            None
        } else {
            Some((self.sensor.0 - remaining, self.sensor.0 + remaining + 1))
        }
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

fn get_ranges(entries: &Vec<Entry>, y: i64) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    for entry in entries {
        if let Some(range) = entry.range_for_y(y) {
            ranges.push(range);
        }
    }
    ranges.sort();
    ranges
}

fn sum_ranges(ranges: &Vec<(i64, i64)>, debug: bool) -> i64 {
    let mut x = i64::MIN;
    let mut sum = 0;
    for range in ranges {
        if debug {
            println!("x = {}", x);
            println!("considering range {:?}", range);
        }
        if range.1 < x {
            if debug {
                println!("skipping");
            }
            continue;
        }
        let start = x.max(range.0);
        if debug {
            println!("adding ({}, {})", start, range.1);
        }
        sum = sum + (range.1 - start);
        x = range.1;
        if debug {
            println!("sum = {}", sum);
        }
    }
    sum
}

fn count_beacons(entries: &Vec<Entry>, y: i64, debug: bool) -> i64 {
    let mut sum = 0;
    let mut seen = HashSet::new();
    for entry in entries {
        if entry.beacon.1 == y {
            if seen.contains(&entry.beacon.0) {
                if debug {
                    println!(
                        "skipping beacon because {} has already been seen",
                        entry.beacon.0
                    );
                }
                continue;
            }
            seen.insert(entry.beacon.0);
            if debug {
                println!(
                    "removing beacon at ({}, {})",
                    entry.beacon.0, entry.beacon.1
                );
            }
            sum = sum + 1;
        }
    }
    sum
}

fn do_part1(entries: &Vec<Entry>, y: i64, debug: bool) {
    let total = sum_ranges(&get_ranges(entries, y), debug);
    let beacons = count_beacons(&entries, y, debug);
    let ans = total - beacons;
    println!("ans = {}", ans);
}

fn process(args: &Args) -> Result<()> {
    let entries = read_input(&args.path, args.debug)?;
    do_part1(&entries, args.y, args.debug);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
