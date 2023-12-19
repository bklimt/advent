use advent::common::split_on;
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Display};
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

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::X => 'x',
                Field::M => 'm',
                Field::A => 'a',
                Field::S => 's',
            }
        )
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
    Move(String),
    Accept,
    Reject,
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
            Consequent::Accept
        } else if s == "R" {
            Consequent::Reject
        } else {
            Consequent::Move(s.to_owned())
        })
    }
}

impl Display for Consequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Consequent::Accept => "A",
                Consequent::Reject => "R",
                Consequent::Move(s) => s,
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Op {
    LessThan,
    GreaterThan,
}

impl Op {
    fn apply(&self, amount1: i64, amount2: i64) -> bool {
        match self {
            Op::LessThan => amount1 < amount2,
            Op::GreaterThan => amount1 > amount2,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::LessThan => '<',
                Op::GreaterThan => '>',
            }
        )
    }
}

#[derive(Debug, Clone)]
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
            ('<', Op::LessThan)
        } else {
            ('>', Op::GreaterThan)
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

impl Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}:{}",
            self.condition_field, self.condition_op, self.condition_amount, self.consequent
        )
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

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: Consequent,
}

impl Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{{", self.name)?;
        for rule in self.rules.iter() {
            write!(f, "{},", rule)?;
        }
        write!(f, "{}}}", self.fallback)
    }
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

#[derive(Debug)]
struct Machine {
    workflows: HashMap<String, Workflow>,
}

impl Machine {
    fn apply(&self, part: &Part) -> Result<bool> {
        let mut state = "in".to_owned();
        loop {
            let workflow = self
                .workflows
                .get(&state)
                .with_context(|| format!("missing state: {}", state))?;
            match workflow.apply(part) {
                Consequent::Accept => {
                    return Ok(true);
                }
                Consequent::Reject => {
                    return Ok(false);
                }
                Consequent::Move(next) => {
                    state = next.clone();
                }
            }
        }
    }

    fn apply_all(&self, parts: &Vec<Part>) -> Result<i64> {
        let mut total = 0;
        for part in parts.iter() {
            if self.apply(part)? {
                total += part.score();
            }
        }
        Ok(total)
    }

    // Does a topological sort of all workflows, based on dependencies.
    fn sort_workflows(&self) -> Vec<Workflow> {
        let mut sorted: Vec<Workflow> = Vec::new();
        let mut sorted_keys: HashSet<String> = HashSet::new();
        let mut unsorted: VecDeque<&Workflow> = VecDeque::new();

        for (_, workflow) in self.workflows.iter() {
            unsorted.push_back(workflow);
        }

        while let Some(workflow) = unsorted.pop_front() {
            let mut leaf = true;
            for rule in workflow.rules.iter() {
                if let Consequent::Move(dep) = &rule.consequent {
                    if !sorted_keys.contains(&dep[..]) {
                        leaf = false;
                        break;
                    }
                }
            }
            if let Consequent::Move(dep) = &workflow.fallback {
                if !sorted_keys.contains(&dep[..]) {
                    leaf = false;
                }
            }
            if leaf {
                sorted.push(workflow.clone());
                sorted_keys.insert(workflow.name.clone());
            } else {
                unsorted.push_back(workflow);
            }
        }

        sorted
    }
}

fn read_input(path: &str) -> Result<(Machine, Vec<Part>)> {
    let mut workflows = HashMap::new();
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
            workflows.insert(workflow.name.clone(), workflow);
        }
    }

    let machine = Machine { workflows };
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
    let ans = machine.apply_all(&parts)?;
    println!("ans1 = {}", ans);

    let sorted = machine.sort_workflows();
    if args.debug {
        for workflow in sorted.iter() {
            println!("{}", workflow);
        }
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
