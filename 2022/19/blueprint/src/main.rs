use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::{HashSet, VecDeque};
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
    max_time: i64,

    #[arg(long)]
    part2: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Part {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const ALL_PARTS: [&Part; 4] = [&Part::Ore, &Part::Clay, &Part::Obsidian, &Part::Geode];

impl Part {
    fn parse(s: &str) -> Result<Self> {
        return if s == "ore" {
            Ok(Self::Ore)
        } else if s == "clay" {
            Ok(Self::Clay)
        } else if s == "obsidian" {
            Ok(Self::Obsidian)
        } else if s == "geode" {
            Ok(Self::Geode)
        } else {
            Err(anyhow!(format!("unknown part: {}", s)))
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Inventory {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn with_part(part: &Part) -> Inventory {
        let mut inv = Inventory::new();
        match part {
            Part::Ore => inv.ore = 1,
            Part::Clay => inv.clay = 1,
            Part::Obsidian => inv.obsidian = 1,
            Part::Geode => inv.geode = 1,
        }
        inv
    }

    fn add(&self, other: &Inventory) -> Inventory {
        Inventory {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn add_part(&self, part: &Part) -> Inventory {
        self.add(&Inventory::with_part(part))
    }

    fn subtract(&self, other: &Inventory) -> Option<Inventory> {
        let inv = Inventory {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        };
        if inv.ore >= 0 && inv.clay >= 0 && inv.obsidian >= 0 && inv.geode >= 0 {
            Some(inv)
        } else {
            None
        }
    }
}

struct Blueprint {
    id: i64,
    ore: Inventory,
    clay: Inventory,
    obsidian: Inventory,
    geode: Inventory,
}

fn parse_ingredient(s: &str) -> Result<(i64, Part)> {
    let space = s.find(' ').ok_or_else(|| anyhow!("missing space: {}", s))?;
    let (samount, s) = s.split_at(space);
    let s = s
        .strip_prefix(" ")
        .ok_or_else(|| anyhow!("unable to strip space: {}", s))?;

    let amount = samount
        .parse::<i64>()
        .with_context(|| anyhow!("invalid number: {}", samount))?;

    println!("    {} {}", amount, s);

    Ok((amount, Part::parse(s)?))
}

fn parse_recipe(s: &str) -> Result<(Part, Inventory)> {
    let s = s
        .strip_prefix("Each ")
        .ok_or_else(|| anyhow!("missing Each: {}", s))?;

    let costs = s
        .find(" robot costs ")
        .ok_or_else(|| anyhow!("missing costs: {}", s))?;
    let (mineral, s) = s.split_at(costs);
    let s = s
        .strip_prefix(" robot costs ")
        .ok_or_else(|| anyhow!("unable to strip costs text: {}", s))?;

    println!("  Each {} robot costs:", mineral);

    let mut cost = Inventory::new();
    for ingredient in s.split(" and ") {
        let ingredient = ingredient.trim();
        let (amount, part) = parse_ingredient(ingredient)?;
        match part {
            Part::Ore => cost.ore = amount,
            Part::Clay => cost.clay = amount,
            Part::Obsidian => cost.obsidian = amount,
            Part::Geode => cost.geode = amount,
        }
    }

    Ok((Part::parse(mineral)?, cost))
}

fn parse_line(s: &str, debug: bool) -> Result<Blueprint> {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 9 obsidian.
    let s = s
        .strip_prefix("Blueprint ")
        .ok_or_else(|| anyhow!("missing Blueprint: {}", s))?;
    let colon = s.find(':').ok_or_else(|| anyhow!("missing colon: {}", s))?;
    let (sid, s) = s.split_at(colon);
    let s = s
        .strip_prefix(": ")
        .ok_or_else(|| anyhow!("unable to strip ore text: {}", s))?;

    let id = sid
        .parse::<i64>()
        .with_context(|| anyhow!("invalid number: {}", sid))?;

    if debug {
        println!("Blueprint {}:", id);
    }
    let mut blueprint = Blueprint {
        id,
        ore: Inventory::new(),
        clay: Inventory::new(),
        obsidian: Inventory::new(),
        geode: Inventory::new(),
    };
    for sentence in s.split('.') {
        if sentence.len() == 0 {
            continue;
        }
        let sentence = sentence.trim();
        let (part, cost) = parse_recipe(sentence)?;
        match part {
            Part::Ore => blueprint.ore = cost,
            Part::Clay => blueprint.clay = cost,
            Part::Obsidian => blueprint.obsidian = cost,
            Part::Geode => blueprint.geode = cost,
        }
    }

    Ok(blueprint)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Plan {
    // What's already been produced.
    inventory: Inventory,
    // What is produced every new second.
    robots: Inventory,
    // How much time has passed.
    time: i64,
}

impl Plan {
    fn new() -> Plan {
        Plan {
            inventory: Inventory::new(),
            robots: Inventory::with_part(&Part::Ore),
            time: 0,
        }
    }

    fn next(&self) -> Plan {
        Plan {
            inventory: self.inventory.add(&self.robots),
            robots: self.robots.clone(),
            time: self.time + 1,
        }
    }

    fn build(&self, part: &Part, cost: &Inventory) -> Option<Plan> {
        if let Some(new_inv) = self.inventory.subtract(cost) {
            Some(Plan {
                inventory: new_inv.add(&self.robots),
                robots: self.robots.add_part(part),
                time: self.time + 1,
            })
        } else {
            None
        }
    }

    fn score_at(&self, time: i64) -> i64 {
        self.inventory.geode + self.robots.geode * (time - self.time)
    }

    fn best_possible_score_at(&self, time: i64) -> i64 {
        let duration = time - self.time;
        let total_from_new_robots = (duration * (duration - 1)) / 2;
        let total_from_existing_robots = self.robots.geode * duration;
        self.inventory.geode + total_from_new_robots + total_from_existing_robots
    }

    fn to_string(&self) -> String {
        format!(
            "Plan{{ inv:[{:?}], robots:[{:?}], time:{} }}",
            self.inventory, self.robots, self.time,
        )
    }
}

impl Blueprint {
    fn recipe(&self, part: &Part) -> &Inventory {
        match part {
            Part::Ore => &self.ore,
            Part::Clay => &self.clay,
            Part::Obsidian => &self.obsidian,
            Part::Geode => &self.geode,
        }
    }

    fn search(&self, max_time: i64, debug: bool) -> i64 {
        let mut max_t = 0;
        let mut best = 0;
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(Plan::new());
        while let Some(plan) = q.pop_front() {
            if plan.time > max_t {
                max_t = plan.time;
                println!("  t = {}", max_t);
            }

            let best_possible_score = plan.best_possible_score_at(max_time);
            if best_possible_score <= best {
                continue;
            }

            let mut hash_plan = plan.clone();
            hash_plan.time = 0;
            if seen.contains(&hash_plan) {
                continue;
            }
            seen.insert(hash_plan);

            let score = plan.score_at(max_time);
            if debug {
                println!("{:10} {} -> {}", q.len(), plan.to_string(), score);
            }
            best = best.max(score);

            if plan.time < max_time {
                q.push_back(plan.next());
                for part in ALL_PARTS {
                    let recipe = self.recipe(part);
                    if let Some(new_plan) = plan.build(part, recipe) {
                        q.push_back(new_plan);
                    }
                }
            }
        }
        best
    }
}

fn read_input(path: &str, debug: bool) -> Result<Vec<Blueprint>> {
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

        if debug {
            println!("line: {}", line);
        }

        v.push(parse_line(line, debug)?);
    }
    Ok(v)
}

fn process(args: &Args) -> Result<()> {
    let mut total = 0;
    let mut product = 1;
    let blueprints = read_input(&args.input, args.debug)?;
    for blueprint in blueprints.iter() {
        if args.part2 && blueprint.id > 3 {
            continue;
        }
        println!("Trying blueprint {}...", blueprint.id);
        let best = blueprint.search(args.max_time, args.debug);
        println!("best = {}", best);
        let quality = blueprint.id * best;
        total += quality;
        product *= best;
    }
    if args.part2 {
        println!("ans = {}", product);
    } else {
        println!("ans = {}", total);
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
