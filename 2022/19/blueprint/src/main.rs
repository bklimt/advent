use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Part {
    Ore,
    Clay,
    Geode,
    Obsidian,
}

const ALL_PARTS: [&Part; 4] = [&Part::Ore, &Part::Clay, &Part::Obsidian, &Part::Geode];

impl Part {
    fn parse(s: &str) -> Result<Self> {
        return if s == "ore" {
            Ok(Self::Ore)
        } else if s == "clay" {
            Ok(Self::Clay)
        } else if s == "geode" {
            Ok(Self::Geode)
        } else if s == "obsidian" {
            Ok(Self::Obsidian)
        } else {
            Err(anyhow!(format!("unknown part: {}", s)))
        };
    }
}

struct Ingredient {
    amount: i32,
    part: Part,
}

struct Recipe {
    part: Part,
    ingredients: Vec<Ingredient>,
}

struct Blueprint {
    id: i32,
    recipes: HashMap<Part, Recipe>,
}

fn parse_ingredient(s: &str) -> Result<Ingredient> {
    let space = s.find(' ').ok_or_else(|| anyhow!("missing space: {}", s))?;
    let (samount, s) = s.split_at(space);
    let s = s
        .strip_prefix(" ")
        .ok_or_else(|| anyhow!("unable to strip space: {}", s))?;

    let amount = samount
        .parse::<i32>()
        .with_context(|| anyhow!("invalid number: {}", samount))?;

    println!("    {} {}", amount, s);

    Ok(Ingredient {
        amount,
        part: Part::parse(s)?,
    })
}

fn parse_recipe(s: &str) -> Result<Recipe> {
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

    let mut v = Vec::new();
    for ingredient in s.split(" and ") {
        let ingredient = ingredient.trim();
        v.push(parse_ingredient(ingredient)?);
    }

    Ok(Recipe {
        part: Part::parse(mineral)?,
        ingredients: v,
    })
}

fn parse_line(s: &str) -> Result<Blueprint> {
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
        .parse::<i32>()
        .with_context(|| anyhow!("invalid number: {}", sid))?;

    let mut recipes = HashMap::new();

    println!("Blueprint {}:", id);
    for sentence in s.split('.') {
        if sentence.len() == 0 {
            continue;
        }
        let sentence = sentence.trim();
        let recipe = parse_recipe(sentence)?;
        recipes.insert(recipe.part.clone(), recipe);
    }

    Ok(Blueprint {
        id: id,
        recipes: recipes,
    })
}

struct Plan {
    // What's already been produced.
    inventory: HashMap<Part, i32>,
    // What is produced every new second.
    robots: HashMap<Part, i32>,
    // How much time has passed.
    time: i32,
}

impl Plan {
    fn new() -> Plan {
        let mut p = Plan {
            inventory: HashMap::new(),
            robots: HashMap::new(),
            time: 0,
        };
        p.robots.insert(Part::Ore, 1);
        p
    }

    fn score_at(&self, time: i32) -> i32 {
        let current_geodes = *self.inventory.get(&Part::Geode).unwrap_or(&0);
        let geode_rate = *self.robots.get(&Part::Geode).unwrap_or(&0);
        current_geodes + geode_rate * (time - self.time)
    }

    fn to_string(&self) -> String {
        let inv_str = self
            .inventory
            .iter()
            .map(|(k, v)| format!("{:?}:{}", k, v))
            .join(",");

        let rob_str = self
            .robots
            .iter()
            .map(|(k, v)| format!("{:?}:{}", k, v))
            .join(",");

        format!(
            "Plan{{ inv:[{}], robots:[{}], time:{} }}",
            inv_str, rob_str, self.time,
        )
    }
}

// Does integer division by taking the ceil of the result.
fn ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

impl Blueprint {
    fn extend(&self, plan: &Plan, part: &Part, max_time: i32) -> Option<Plan> {
        if plan.time >= max_time {
            return None;
        }
        // println!("Considering extending {} with {:?}", plan.to_string(), part);

        // How long would it take to get the inventory to build that?
        let mut wait = 0;
        let recipe = self.recipes.get(part).unwrap();
        for ingredient in recipe.ingredients.iter() {
            let have = *plan.inventory.get(&ingredient.part).unwrap_or(&0);
            if have >= ingredient.amount {
                // We already have enough of this ingredient.
                continue;
            }
            let rate = *plan.robots.get(&ingredient.part).unwrap_or(&0);
            if rate == 0 {
                // We aren't making this yet, so we'll never have enough by waiting.
                return None;
            }
            let new_wait = ceil_div(ingredient.amount - have, rate);
            if plan.time + new_wait >= max_time {
                // It would take longer than we have.
                return None;
            }
            wait = wait.max(new_wait);
        }

        // println!("can build {:?} robot after {} seconds", part, wait);

        // The 1 is the time to build the robot.
        let mut new_plan = Plan {
            inventory: plan.inventory.clone(),
            robots: plan.robots.clone(),
            time: plan.time + wait + 1,
        };

        // Update the inventory first.
        for part in ALL_PARTS {
            let previous = *new_plan.inventory.get(part).unwrap_or(&0);
            let rate = *new_plan.robots.get(part).unwrap_or(&0);
            let new_amount = previous + rate * wait;
            if new_amount == 0 {
                new_plan.inventory.remove(&part);
            } else {
                new_plan.inventory.insert(part.clone(), new_amount);
            }
        }

        // Now remove the resources used to build the robot.
        for ingredient in recipe.ingredients.iter() {
            let previous = *new_plan.inventory.get(&ingredient.part).unwrap_or(&0);
            let needed = ingredient.amount;
            if needed > previous {
                panic!("recipe needed more than it had");
            }
            if previous - needed == 0 {
                new_plan.inventory.remove(&ingredient.part);
            } else {
                new_plan
                    .inventory
                    .insert(ingredient.part.clone(), previous - needed);
            }
        }

        // Finally, add the new robot.
        new_plan
            .robots
            .insert(part.clone(), 1 + *new_plan.robots.get(part).unwrap_or(&0));

        // println!("new plan is {}", new_plan.to_string());

        Some(new_plan)
    }

    fn search(&self, max_time: i32) -> i32 {
        let mut best = 0;
        let mut q = VecDeque::new();
        q.push_back(Plan::new());
        while let Some(plan) = q.pop_front() {
            let score = plan.score_at(max_time);
            println!("{} -> {}", plan.to_string(), score);
            best = best.max(score);
            for part in ALL_PARTS {
                if let Some(new_plan) = self.extend(&plan, part, max_time) {
                    q.push_back(new_plan);
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

        v.push(parse_line(line)?);
    }
    Ok(v)
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let blueprints = read_input(&args.input, args.debug)?;
    for blueprint in blueprints.iter() {
        println!("Trying blueprint {}...", blueprint.id);
        let ans = blueprint.search(24);
        println!("ans = {}", ans);
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
