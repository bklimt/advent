use anyhow::{anyhow, Context, Result};
use clap::Parser;
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

struct Path {
    path: Vec<String>,
    time: i32,
    flow: i32,
    total: i32,
}

impl Path {
    fn score(&self, time: i32) -> i32 {
        self.total + (self.flow * (time - self.time))
    }
}

fn extend(
    path: &Path,
    next: &Valve,
    max_time: i32,
    seen: &HashSet<String>,
    adj: &HashMap<(String, String), i32>,
) -> Option<Path> {
    return if next.rate == 0 {
        None
    } else if seen.contains(&next.name) {
        None
    } else {
        let current = path.path.last().unwrap();
        let edge = (current.clone(), next.name.clone());
        if let Some(dist) = adj.get(&edge) {
            let cost = dist + 1;
            let new_time = path.time + cost;
            if new_time >= max_time {
                None
            } else {
                // It passed all the tests. Try it.
                let new_total = path.total + (cost * path.flow);
                let new_flow = path.flow + next.rate;

                let mut new_path = path.path.clone();
                new_path.push(next.name.clone());

                Some(Path {
                    path: new_path,
                    time: new_time,
                    flow: new_flow,
                    total: new_total,
                })
            }
        } else {
            None
        }
    };
}

fn bfs_search1(
    valves: &HashMap<String, Valve>,
    adj: &HashMap<(String, String), i32>,
    max_time: i32,
    debug: bool,
) -> i32 {
    struct Candidate {
        path: Path,
        seen: HashSet<String>,
    }
    let empty = Candidate {
        path: Path {
            path: vec!["AA".to_string()],
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: HashSet::new(),
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
                    candidate.path.path.join(" -> "),
                    candidate.path.time,
                    candidate.path.flow,
                    candidate.path.total,
                );
            }

            // Consider all the next steps.
            for (_, next) in valves.iter() {
                if let Some(new_path) =
                    extend(&candidate.path, next, max_time, &candidate.seen, adj)
                {
                    best = best.max(new_path.score(max_time));
                    more = true;

                    let mut new_seen = candidate.seen.clone();
                    new_seen.insert(next.name.clone());

                    new_candidates.push(Candidate {
                        path: new_path,
                        seen: new_seen,
                    });
                }
            }
        }
        candidates = new_candidates;
    }
    best
}

fn bfs_search2(
    valves: &HashMap<String, Valve>,
    adj: &HashMap<(String, String), i32>,
    max_time: i32,
    debug: bool,
) -> i32 {
    struct Candidate {
        human: Path,
        elephant: Path,
        seen: HashSet<String>,
    }
    let empty = Candidate {
        human: Path {
            path: vec!["AA".to_string()],
            time: 0,
            flow: 0,
            total: 0,
        },
        elephant: Path {
            path: vec!["AA".to_string()],
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: HashSet::new(),
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
                    "Considering hum {} [time={}, flow={}, total={}]",
                    candidate.human.path.join(" -> "),
                    candidate.human.time,
                    candidate.human.flow,
                    candidate.human.total,
                );
                println!(
                    "Considering ele {} [time={}, flow={}, total={}]",
                    candidate.elephant.path.join(" -> "),
                    candidate.elephant.time,
                    candidate.elephant.flow,
                    candidate.elephant.total,
                );
            }

            // Consider all the next steps.
            for (_, next) in valves.iter() {
                if let Some(new_human) =
                    extend(&candidate.human, next, max_time, &candidate.seen, adj)
                {
                    best = best.max(new_human.score(max_time) + candidate.elephant.score(max_time));
                    more = true;

                    let mut new_seen = candidate.seen.clone();
                    new_seen.insert(next.name.clone());

                    let new_elephant = Path {
                        path: candidate.elephant.path.clone(),
                        time: candidate.elephant.time,
                        flow: candidate.elephant.flow,
                        total: candidate.elephant.total,
                    };

                    new_candidates.push(Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    });
                }
                if let Some(new_elephant) =
                    extend(&candidate.elephant, next, max_time, &candidate.seen, adj)
                {
                    best = best.max(new_elephant.score(max_time) + candidate.human.score(max_time));
                    more = true;

                    let mut new_seen = candidate.seen.clone();
                    new_seen.insert(next.name.clone());

                    let new_human = Path {
                        path: candidate.human.path.clone(),
                        time: candidate.human.time,
                        flow: candidate.human.flow,
                        total: candidate.human.total,
                    };

                    new_candidates.push(Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    });
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

    let ans = if args.part2 {
        bfs_search2(&valves, &adj, 26, args.debug)
    } else {
        bfs_search1(&valves, &adj, 30, args.debug)
    };
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
