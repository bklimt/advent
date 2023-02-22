use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
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

#[derive(Debug)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Vec<String>,
    open: bool,
}

impl Valve {
    fn parse(s: &str) -> Result<Valve> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let s = s
            .strip_prefix("Valve ")
            .ok_or_else(|| anyhow!("missing 'Valve': {}", s))?;
        let space = s
            .find(' ')
            .ok_or_else(|| anyhow!("missing first space: {}", s))?;
        let (name, s) = s.split_at(space);
        let s = s
            .strip_prefix(" has flow rate=")
            .ok_or_else(|| anyhow!("missing ' has flow rate=': {}", s))?;
        let semi = s
            .find(';')
            .ok_or_else(|| anyhow!("missing semicolon: {}", s))?;
        let (srate, s) = s.split_at(semi);

        let tunnels = if s.starts_with("; tunnel leads to valve ") {
            let s = s.trim_start_matches("; tunnel leads to valve ");
            vec![s.to_string()]
        } else if s.starts_with("; tunnels lead to valves ") {
            let s = s.trim_start_matches("; tunnels lead to valves ");
            s.split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        } else {
            return Err(anyhow!("missing tunnels section: {}", s));
        };

        let rate = srate
            .parse::<i32>()
            .with_context(|| anyhow!("invalid number: {}", srate))?;

        Ok(Valve {
            name: name.to_string(),
            rate,
            tunnels,
            open: false,
        })
    }
}

fn read_input(path: &str, debug: bool) -> Result<HashMap<String, Valve>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut valves = HashMap::new();
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
        let valve = Valve::parse(line)?;
        if debug {
            println!("valve: {:?}", valve);
        }
        valves.insert(valve.name.clone(), valve);
    }
    Ok(valves)
}

fn process(args: &Args) -> Result<()> {
    let _valves = read_input(&args.path, args.debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
