use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
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
    max_time: i32,

    #[arg(long)]
    debug: bool,
}

#[derive(Debug)]
struct Valve {
    id: i64,
    name: String,
    rate: i32,
    tunnels: Vec<i64>,
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

        let stunnels = if s.starts_with("; tunnel leads to valve ") {
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

        let id = i64::from_str_radix(name, 36)
            .with_context(|| format!("unable to parse name: {}", name))?;

        let mut tunnels = Vec::new();
        for tunnel in stunnels {
            let tid = i64::from_str_radix(tunnel.as_str(), 36)
                .with_context(|| format!("unable to parse tunnel: {}", tunnel))?;
            tunnels.push(tid);
        }

        Ok(Valve {
            id,
            name: name.to_string(),
            rate,
            tunnels,
        })
    }
}

fn read_input(path: &str, debug: bool) -> Result<HashMap<i64, Valve>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut useful = 0;
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
        if valve.rate > 0 {
            useful = useful + 1;
        }
        valves.insert(valve.id, valve);
    }
    if debug {
        println!("{} useful valves", useful);
    }
    Ok(valves)
}

fn build_adj(valves: &HashMap<i64, Valve>, debug: bool) -> HashMap<(i64, i64), i32> {
    let mut adj = HashMap::new();
    for (_, v1) in valves.iter() {
        for v2 in v1.tunnels.iter() {
            adj.insert((v1.id, *v2), 1);
        }
    }
    for (mid, _) in valves.iter() {
        for (start, _) in valves.iter() {
            if start == mid {
                continue;
            }
            let p1 = (*start, *mid);
            for (end, _) in valves.iter() {
                if start == end || mid == end {
                    continue;
                }
                let p = (*start, *end);
                let p2 = (*mid, *end);
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
    path: Vec<i64>,
    time: i32,
    flow: i32,
    total: i32,
}

impl Path {
    fn score(&self, time: i32) -> i32 {
        self.total + (self.flow * (time - self.time))
    }

    fn string_from_id(id: &i64) -> String {
        format!(
            "{}{}",
            char::from_digit((id / 36) as u32, 36).unwrap(),
            char::from_digit((id % 36) as u32, 36).unwrap(),
        )
    }

    fn to_string(&self) -> String {
        self.path.iter().map(Path::string_from_id).join(" -> ")
    }
}

fn extend(
    path: &Path,
    next: &Valve,
    max_time: i32,
    seen: &Vec<i64>,
    adj: &HashMap<(i64, i64), i32>,
) -> Option<Path> {
    return if next.rate == 0 {
        None
    } else if seen.contains(&next.id) {
        None
    } else {
        let current = path.path.last().unwrap();
        let edge = (*current, next.id);
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
                new_path.push(next.id);

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
    valves: &HashMap<i64, Valve>,
    adj: &HashMap<(i64, i64), i32>,
    max_time: i32,
    debug: bool,
) -> Result<i32> {
    struct Candidate {
        path: Path,
        seen: Vec<i64>,
    }
    let aa = i64::from_str_radix("AA", 36).with_context(|| "unable to parse AA")?;
    let empty = Candidate {
        path: Path {
            path: vec![aa],
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: Vec::new(),
    };
    let mut candidates = VecDeque::new();
    candidates.push_back(empty);

    let mut best = 0;
    let mut total = 1;

    while let Some(candidate) = candidates.pop_front() {
        if debug {
            println!(
                "{} {} Considering {} [time={}, flow={}, total={}]",
                total,
                candidates.len(),
                candidate.path.to_string(),
                candidate.path.time,
                candidate.path.flow,
                candidate.path.total,
            );
        }

        // Consider all the next steps.
        for (_, next) in valves.iter() {
            if let Some(new_path) = extend(&candidate.path, next, max_time, &candidate.seen, adj) {
                best = best.max(new_path.score(max_time));

                let mut new_seen = candidate.seen.clone();
                new_seen.push(next.id);

                total = total + 1;
                candidates.push_back(Candidate {
                    path: new_path,
                    seen: new_seen,
                });
            }
        }
    }
    Ok(best)
}

fn bfs_search2(
    valves: &HashMap<i64, Valve>,
    adj: &HashMap<(i64, i64), i32>,
    max_time: i32,
    debug: bool,
) -> Result<i32> {
    struct Candidate {
        human: Path,
        elephant: Path,
        seen: Vec<i64>,
    }
    let aa = i64::from_str_radix("AA", 36).with_context(|| "unable to parse AA")?;
    let empty = Candidate {
        human: Path {
            path: vec![aa],
            time: 0,
            flow: 0,
            total: 0,
        },
        elephant: Path {
            path: vec![aa],
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: Vec::new(),
    };
    let mut candidates = VecDeque::new();
    candidates.push_back(empty);

    let mut best = 0;
    let mut total = 1;

    while let Some(candidate) = candidates.pop_front() {
        if debug {
            println!(
                "{} {} Considering hum {} [time={}, flow={}, total={}] \n                                                                            ele {} [time={}, flow={}, total={}]",
                total,
                candidates.len(),
                candidate.human.to_string(),
                candidate.human.time,
                candidate.human.flow,
                candidate.human.total,
                candidate.elephant.to_string(),
                candidate.elephant.time,
                candidate.elephant.flow,
                candidate.elephant.total,
            );
        }

        let human_path = candidate.human.to_string();
        let elephant_path = candidate.elephant.to_string();

        // Consider all the next steps.
        for (_, next) in valves.iter() {
            if let Some(new_human) = extend(&candidate.human, next, max_time, &candidate.seen, adj)
            {
                let new_human_path = new_human.to_string();
                if elephant_path < new_human_path {
                    if debug {
                        println!("skipping because {} < {}", elephant_path, new_human_path);
                    }
                } else {
                    best = best.max(new_human.score(max_time) + candidate.elephant.score(max_time));

                    let mut new_seen = candidate.seen.clone();
                    new_seen.push(next.id);

                    let new_elephant = Path {
                        path: candidate.elephant.path.clone(),
                        time: candidate.elephant.time,
                        flow: candidate.elephant.flow,
                        total: candidate.elephant.total,
                    };

                    total = total + 1;
                    candidates.push_back(Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    });
                }
            }

            if let Some(new_elephant) =
                extend(&candidate.elephant, next, max_time, &candidate.seen, adj)
            {
                let new_elephant_path = new_elephant.to_string();
                if new_elephant_path < human_path {
                    if debug {
                        println!("skipping because {} < {}", new_elephant_path, human_path);
                    }
                } else {
                    best = best.max(new_elephant.score(max_time) + candidate.human.score(max_time));

                    let mut new_seen = candidate.seen.clone();
                    new_seen.push(next.id);

                    let new_human = Path {
                        path: candidate.human.path.clone(),
                        time: candidate.human.time,
                        flow: candidate.human.flow,
                        total: candidate.human.total,
                    };

                    candidates.push_back(Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    });
                }
            }
        }
    }
    Ok(best)
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let valves = read_input(&args.path, args.debug)?;

    println!("building adjacency matrix...");
    let adj = build_adj(&valves, args.debug);

    println!("searching...");
    let ans = if args.part2 {
        bfs_search2(&valves, &adj, args.max_time, args.debug)?
    } else {
        bfs_search1(&valves, &adj, args.max_time, args.debug)?
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
