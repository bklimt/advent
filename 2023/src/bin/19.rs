use advent::common::split_on;
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            bail!("invalid field: {}", s);
        }
        match s.chars().nth(0) {
            Some('x') => Ok(Field::X),
            Some('m') => Ok(Field::M),
            Some('a') => Ok(Field::A),
            Some('s') => Ok(Field::S),
            _ => Err(anyhow!("invalid field: {}", s)),
        }
    }
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn score(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

impl Index<Field> for Part {
    type Output = i64;

    fn index(&self, index: Field) -> &Self::Output {
        match index {
            Field::X => &self.x,
            Field::M => &self.m,
            Field::A => &self.a,
            Field::S => &self.s,
        }
    }
}

impl IndexMut<Field> for Part {
    fn index_mut(&mut self, index: Field) -> &mut Self::Output {
        match index {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !input.starts_with("{x=") {
            bail!("missing x: {}", input);
        }
        let rest = &input[3..];
        let (x, rest) = split_on(rest, ',').context(format!("missing first comma: {}", input))?;

        if !rest.starts_with("m=") {
            bail!("missing m: {}", input);
        }
        let rest = &rest[2..];
        let (m, rest) = split_on(rest, ',').context(format!("missing second comma: {}", input))?;

        if !rest.starts_with("a=") {
            bail!("missing a: {}", input);
        }
        let rest = &rest[2..];
        let (a, rest) = split_on(rest, ',').context(format!("missing third comma: {}", input))?;

        if !rest.starts_with("s=") {
            bail!("missing s: {}", input);
        }
        let rest = &rest[2..];
        let (s, rest) = split_on(rest, '}').context(format!("missing closing brace: {}", input))?;

        if rest.len() != 0 {
            bail!("trailing chars: {}", input);
        }

        let x = x.parse()?;
        let m = m.parse()?;
        let a = a.parse()?;
        let s = s.parse()?;

        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug, Clone)]
enum Consequent {
    MOVE(String),
    ACCEPT,
    REJECT,
}

impl FromStr for Consequent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            bail!("invalid consequent: {}", s);
        }
        if s.chars().any(|c| !c.is_alphabetic()) {
            bail!("invalid consequent: {}", s);
        }
        Ok(if s == "A" {
            Consequent::ACCEPT
        } else if s == "R" {
            Consequent::REJECT
        } else {
            Consequent::MOVE(s.to_owned())
        })
    }
}

#[derive(Debug)]
enum Op {
    LT,
    GT,
}

impl Op {
    fn apply(&self, amount1: i64, amount2: i64) -> bool {
        match self {
            Op::LT => amount1 < amount2,
            Op::GT => amount1 > amount2,
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition_field: Field,
    condition_op: Op,
    condition_amount: i64,
    consequent: Consequent,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antecedent, consequent) = split_on(s, ':').context(format!("invalid rule: {}", s))?;
        let consequent = consequent.parse()?;

        let (op, condition_op) = if antecedent.contains('<') {
            ('<', Op::LT)
        } else {
            ('>', Op::GT)
        };

        let (field, amount) = split_on(antecedent, op).context(format!("invalid rule: {}", s))?;
        let condition_field = field.parse()?;
        let condition_amount = amount.parse()?;

        Ok(Rule {
            condition_field,
            condition_op,
            condition_amount,
            consequent,
        })
    }
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<&Consequent> {
        let amount1 = part[self.condition_field];
        let amount2 = self.condition_amount;
        if self.condition_op.apply(amount1, amount2) {
            Some(&self.consequent)
        } else {
            None
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: Consequent,
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = split_on(s, '{').context(format!("invalid workflow: {}", s))?;
        let name = name.to_owned();
        if !rules.ends_with('}') {
            bail!("invalid workflow: {}", s);
        }
        let rules = &rules[..rules.len() - 1];

        let mut rules_str = rules;
        let mut rules = Vec::new();
        while let Some((rule, rest)) = split_on(rules_str, ',') {
            rules.push(rule.parse()?);
            rules_str = rest;
        }
        let fallback = rules_str.parse()?;

        Ok(Workflow {
            name,
            rules,
            fallback,
        })
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Consequent {
        for rule in self.rules.iter() {
            if let Some(result) = rule.apply(part) {
                return result;
            }
        }
        &self.fallback
    }
}

fn apply(machine: &HashMap<String, Workflow>, part: &Part) -> Result<bool> {
    let mut state = "in".to_owned();
    loop {
        let workflow = machine
            .get(&state)
            .with_context(|| format!("missing state: {}", state))?;
        match workflow.apply(part) {
            Consequent::ACCEPT => {
                return Ok(true);
            }
            Consequent::REJECT => {
                return Ok(false);
            }
            Consequent::MOVE(next) => {
                state = next.clone();
            }
        }
    }
}

fn apply_all(machine: &HashMap<String, Workflow>, parts: &Vec<Part>) -> Result<i64> {
    let mut total = 0;
    for part in parts.iter() {
        if apply(machine, part)? {
            total += part.score();
        }
    }
    Ok(total)
}

fn read_input(path: &str) -> Result<(HashMap<String, Workflow>, Vec<Part>)> {
    let mut machine = HashMap::new();
    let mut parts = Vec::new();

    let f = File::open(path)?;
    let mut f = BufReader::new(f);
    let mut reading_parts = false;
    loop {
        let mut line = String::new();
        let n = f.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                break;
            }
            reading_parts = true;
            continue;
        }

        if reading_parts {
            parts.push(line.parse()?);
        } else {
            let workflow: Workflow = line.parse()?;
            machine.insert(workflow.name.clone(), workflow);
        }
    }

    Ok((machine, parts))
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
    let (machine, parts) = read_input(args.input.as_str())?;
    let ans = apply_all(&machine, &parts)?;
    println!("ans1 = {}", ans);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
