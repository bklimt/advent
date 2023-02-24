use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
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

struct Ingredient {
    amount: i32,
    part: String,
}

struct Recipe {
    part: String,
    ingredients: Vec<Ingredient>,
}

struct Blueprint {
    id: i32,
    recipes: HashMap<String, Recipe>,
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
        part: s.to_string(),
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
        part: mineral.to_string(),
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
    let _ = read_input(&args.input, args.debug)?;

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
