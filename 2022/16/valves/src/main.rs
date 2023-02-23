use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
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

        Ok(Valve { id, rate, tunnels })
    }
}

fn get_useful_valves(valves: &HashMap<i64, Valve>) -> Vec<Valve> {
    let mut useful = Vec::new();
    for (_, valve) in valves.iter() {
        if valve.rate > 0 {
            useful.push(Valve {
                id: valve.id,
                rate: valve.rate,
                tunnels: valve.tunnels.clone(),
            });
        }
    }
    useful
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
}

impl Path {
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

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == Ordering::Equal
    }
}

impl Eq for Path {}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

fn compare(left: &Path, right: &Path) -> Ordering {
    let len = left.path.len().min(right.path.len());
    for i in 0..len {
        if left.path[i] < right.path[i] {
            return Ordering::Less;
        } else if left.path[i] > right.path[i] {
            return Ordering::Greater;
        }
    }
    return left.path.len().cmp(&right.path.len());
}

#[derive(PartialEq, Eq, Hash)]
struct ScoredPath {
    path: Path,
    time: i32,
    flow: i32,
    total: i32,
}

impl ScoredPath {
    fn score(&self, time: i32) -> i32 {
        self.total + (self.flow * (time - self.time))
    }
}

fn extend(
    path: &ScoredPath,
    next: &Valve,
    max_time: i32,
    seen: &Vec<i64>,
    adj: &HashMap<(i64, i64), i32>,
) -> Option<ScoredPath> {
    return if next.rate == 0 {
        None
    } else if seen.contains(&next.id) {
        None
    } else {
        let current = path.path.path.last().unwrap();
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

                let mut new_path = Path {
                    path: path.path.path.clone(),
                };
                new_path.path.push(next.id);

                Some(ScoredPath {
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
    valves: &Vec<Valve>,
    adj: &HashMap<(i64, i64), i32>,
    max_time: i32,
    debug: bool,
) -> Result<i32> {
    let mut best_scores = HashMap::new();

    #[derive(PartialEq, Eq, Hash)]
    struct Candidate {
        path: ScoredPath,
        seen: Vec<i64>,
    }
    let aa = i64::from_str_radix("AA", 36).with_context(|| "unable to parse AA")?;
    let empty = Candidate {
        path: ScoredPath {
            path: Path { path: vec![aa] },
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: Vec::new(),
    };
    let mut candidates = PriorityQueue::new();
    candidates.push(empty, 0);

    let mut best = 0;
    let mut total = 1;

    while let Some(candidate_score) = candidates.pop() {
        let (candidate, _) = candidate_score;
        if debug {
            println!(
                "{} {} Considering {} [time={}, flow={}, total={}]",
                total,
                candidates.len(),
                candidate.path.path.to_string(),
                candidate.path.time,
                candidate.path.flow,
                candidate.path.total,
            );
        }

        // Consider all the next steps.
        for next in valves.iter() {
            if let Some(new_path) = extend(&candidate.path, next, max_time, &candidate.seen, adj) {
                let score = new_path.score(max_time);
                best = best.max(score);

                let mut new_seen = candidate.seen.clone();
                new_seen.push(next.id);

                let pos = next.id;

                total = total + 1;
                let new_candidate = Candidate {
                    path: new_path,
                    seen: new_seen,
                };

                let is_best =
                    if let Some(best_score) = best_scores.get(&(pos, new_candidate.path.time)) {
                        score >= *best_score
                    } else {
                        true
                    };

                if is_best {
                    for t in new_candidate.path.time..max_time {
                        best_scores.insert((pos, t), score);
                    }
                    candidates.push(new_candidate, score);
                }
            }
        }
    }
    Ok(best)
}

fn bfs_search2(
    valves: &Vec<Valve>,
    adj: &HashMap<(i64, i64), i32>,
    max_time: i32,
    debug: bool,
) -> Result<i32> {
    let mut best_scores = HashMap::new();

    #[derive(PartialEq, Eq, Hash)]
    struct Candidate {
        human: ScoredPath,
        elephant: ScoredPath,
        seen: Vec<i64>,
    }
    let aa = i64::from_str_radix("AA", 36).with_context(|| "unable to parse AA")?;
    let empty = Candidate {
        human: ScoredPath {
            path: Path { path: vec![aa] },
            time: 0,
            flow: 0,
            total: 0,
        },
        elephant: ScoredPath {
            path: Path { path: vec![aa] },
            time: 0,
            flow: 0,
            total: 0,
        },
        seen: Vec::new(),
    };
    let mut candidates = PriorityQueue::new();
    candidates.push(empty, 0);

    let mut best = 0;
    let mut total = 1;

    while let Some(candidate_score) = candidates.pop() {
        let (candidate, _) = candidate_score;
        if debug {
            println!(
                "{} {} hum {} [time={}, flow={}, total={}]",
                total,
                candidates.len(),
                candidate.human.path.to_string(),
                candidate.human.time,
                candidate.human.flow,
                candidate.human.total,
            );
            println!(
                "{} {} ele {} [time={}, flow={}, total={}]",
                total,
                candidates.len(),
                candidate.elephant.path.to_string(),
                candidate.elephant.time,
                candidate.elephant.flow,
                candidate.elephant.total,
            );
        }

        // Consider all the next steps.
        for next in valves.iter() {
            let mut new_candidates = Vec::new();

            if let Some(new_human) = extend(&candidate.human, next, max_time, &candidate.seen, adj)
            {
                if candidate.elephant.path >= new_human.path {
                    let score = new_human.score(max_time) + candidate.elephant.score(max_time);
                    best = best.max(score);

                    let mut new_seen = candidate.seen.clone();
                    new_seen.push(next.id);

                    let new_elephant = ScoredPath {
                        path: Path {
                            path: candidate.elephant.path.path.clone(),
                        },
                        time: candidate.elephant.time,
                        flow: candidate.elephant.flow,
                        total: candidate.elephant.total,
                    };

                    let new_candidate = Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    };
                    new_candidates.push(new_candidate);
                }
            }

            if let Some(new_elephant) =
                extend(&candidate.elephant, next, max_time, &candidate.seen, adj)
            {
                if new_elephant.path >= candidate.human.path {
                    let mut new_seen = candidate.seen.clone();
                    new_seen.push(next.id);

                    let new_human = ScoredPath {
                        path: Path {
                            path: candidate.human.path.path.clone(),
                        },
                        time: candidate.human.time,
                        flow: candidate.human.flow,
                        total: candidate.human.total,
                    };

                    let new_candidate = Candidate {
                        human: new_human,
                        elephant: new_elephant,
                        seen: new_seen,
                    };

                    new_candidates.push(new_candidate);
                }
            }

            for new_candidate in new_candidates {
                total = total + 1;

                let score =
                    new_candidate.elephant.score(max_time) + new_candidate.human.score(max_time);
                best = best.max(score);

                let mut was_best = false;

                for t_human in new_candidate.human.time..max_time {
                    for t_elephant in new_candidate.elephant.time..max_time {
                        let pos = (
                            *new_candidate.human.path.path.last().unwrap(),
                            *new_candidate.elephant.path.path.last().unwrap(),
                            t_human,
                            t_elephant,
                        );

                        let is_best = if let Some(best_score) = best_scores.get(&pos) {
                            score >= *best_score
                        } else {
                            true
                        };

                        if is_best {
                            was_best = true;
                            best_scores.insert(pos.clone(), score);
                        }
                    }
                }

                if was_best {
                    candidates.push(new_candidate, score);
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

    let useful = get_useful_valves(&valves);

    println!("searching...");
    let ans = if args.part2 {
        bfs_search2(&useful, &adj, args.max_time, args.debug)?
    } else {
        bfs_search1(&useful, &adj, args.max_time, args.debug)?
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
