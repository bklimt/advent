use advent::common::{read_lines, StrIterator};
use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    typ: ModuleType,
    name: String,
    destinations: Vec<String>,
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .find(" -> ")
            .with_context(|| format!("module missing ->: {:?}", s))?;
        let lhs = &s[..pos];
        let rhs = &s[pos + 4..];
        let c = s
            .chars()
            .nth(0)
            .with_context(|| format!("module lhs is empty: {:?}", s))?;
        let (typ, name) = match c {
            '%' => (ModuleType::FlipFlop, &lhs[1..]),
            '&' => (ModuleType::Conjunction, &lhs[1..]),
            _ => (ModuleType::Broadcaster, lhs),
        };
        let name = name.to_owned();
        let destinations: Vec<String> = rhs.split(", ").map(|s| s.to_owned()).collect_vec();
        Ok(Module {
            typ,
            name,
            destinations,
        })
    }
}

fn read_input(path: &str) -> Result<HashMap<String, Module>> {
    let mut map = HashMap::new();
    let modules: Vec<Module> = read_lines(path)?.parse_all()?;
    for module in modules {
        map.insert(module.name.clone(), module);
    }
    Ok(map)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(args.input.as_str())?;
    for (_, module) in input.iter() {
        println!("{:?}", module);
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
