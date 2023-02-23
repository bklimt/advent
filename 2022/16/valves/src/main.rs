use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

fn build_adj(valves: &HashMap<String, Valve>, debug: bool) -> HashMap<(String, String), i32> {
    let mut adj = HashMap::new();
    for (_, v1) in valves.iter() {
        for v2 in v1.tunnels.iter() {
            adj.insert((v1.name.clone(), v2.clone()), 1);
        }
    }
    for _ in valves.iter() {
        for (start, _) in valves.iter() {
            for (end, _) in valves.iter() {
                if start == end {
                    continue;
                }
                let p = (start.clone(), end.clone());
                for (mid, _) in valves.iter() {
                    if start == mid || mid == end {
                        continue;
                    }
                    let p1 = (start.clone(), mid.clone());
                    let p2 = (mid.clone(), end.clone());
                    if let Some(d1) = adj.get(&p1) {
                        if let Some(d2) = adj.get(&p2) {
                            if let Some(d) = adj.get(&p) {
                                if *d1 + *d2 < *d {
                                    adj.insert(p.clone(), d1 + d2);
                                }
                            } else {
                                adj.insert(p.clone(), d1 + d2);
                            }
                        }
                    }
                }
            }
        }
    }
    for (n, _) in valves.iter() {
        adj.insert((n.clone(), n.clone()), 0);
    }
    if debug {
        for ((start, end), d) in adj.iter() {
            println!("{} -> {} = {}", start, end, d);
        }
    }
    adj
}

fn dfs_search(
    valves: &HashMap<String, Valve>,
    adj: &HashMap<(String, String), i32>,
    current: &String,
    flow: i32,
    time_remaining: i32,
    open: &mut HashSet<String>,
    debug: bool,
) -> i32 {
    if time_remaining < 0 {
        panic!("invalid time_remaining: {}", 0);
    }
    if time_remaining == 0 {
        if debug {
            println!("skipping {} with time = 0", current);
        }
        return 0;
    }

    if debug {
        println!("visiting {} with time = {}", current, time_remaining);
    }

    // The default is to do nothing.
    let mut best = flow * time_remaining;

    for (_, next) in valves.iter() {
        if next.rate == 0 {
            continue;
        }
        if open.contains(&next.name) {
            continue;
        }
        let edge = (current.clone(), next.name.clone());
        if let Some(dist) = adj.get(&edge) {
            let cost = dist + 1;
            let new_time_remaining = time_remaining - cost;
            if new_time_remaining < 0 {
                continue;
            }

            // It passed all the tests. Try it.
            let addition = cost * flow;
            let new_flow = flow + next.rate;

            open.insert(next.name.clone());
            best = best.max(
                addition
                    + dfs_search(
                        valves,
                        adj,
                        &next.name,
                        new_flow,
                        new_time_remaining,
                        open,
                        debug,
                    ),
            );
            open.remove(&next.name);
        }
    }

    if debug {
        println!("returning best flow = {}", best);
    }
    best
}

fn bfs_search(
    valves: &HashMap<String, Valve>,
    adj: &HashMap<(String, String), i32>,
    debug: bool,
) -> i32 {
    struct Candidate {
        path: Vec<String>,
        time: i32,
        flow: i32,
        total: i32,
    }
    let empty = Candidate {
        path: vec!["AA".to_string()],
        time: 0,
        flow: 0,
        total: 0,
    };
    let mut candidates = Vec::new();
    candidates.push(empty);
    let mut more = true;

    let mut best = 0;

    while more {
        more = false;

        // Try extending any current candidate.
        let mut new_candidates = Vec::new();
        for candidate in candidates.iter() {
            if debug {
                println!(
                    "Considering {} [time={}, flow={}, total={}]",
                    candidate.path.join(" -> "),
                    candidate.time,
                    candidate.flow,
                    candidate.total,
                );
            }

            // Consider all the next steps.
            for (_, next) in valves.iter() {
                if next.rate == 0 {
                    continue;
                }
                if candidate.path[1..].contains(&next.name) {
                    continue;
                }
                let current = candidate.path.last().unwrap();
                let edge = (current.clone(), next.name.clone());
                if let Some(dist) = adj.get(&edge) {
                    let cost = dist + 1;
                    let new_time = cost + candidate.time;
                    if new_time >= 30 {
                        continue;
                    }

                    // It passed all the tests. Try it.
                    let addition = cost * candidate.flow;
                    let new_total = candidate.total + addition;
                    let new_flow = candidate.flow + next.rate;

                    let mut new_path = candidate.path.clone();
                    new_path.push(next.name.clone());

                    new_candidates.push(Candidate {
                        path: new_path,
                        time: new_time,
                        flow: new_flow,
                        total: new_total,
                    });

                    let score = new_total + (new_flow * (30 - new_time));
                    best = best.max(score);

                    more = true;
                }
            }
        }
        candidates = new_candidates;
    }
    best
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let valves = read_input(&args.path, args.debug)?;

    println!("building adjacency matrix...");
    let adj = build_adj(&valves, args.debug);

    /*
    println!("doing depth-first search...");
    let current = "AA".to_string();
    let current_flow = 0;
    let time = 30;
    let mut open = HashSet::new();
    let flow = dfs_search(
        &valves,
        &adj,
        &current,
        current_flow,
        time,
        &mut open,
        args.debug,
    );
    */

    let ans = bfs_search(&valves, &adj, args.debug);
    println!("ans = {}", ans);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
