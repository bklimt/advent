use advent::common::{read_lines, StrIterator};
use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum SignalLevel {
    Low,
    High,
}

impl Display for SignalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SignalLevel::Low => "low",
                SignalLevel::High => "high",
            }
        )
    }
}

#[derive(Debug)]
struct Signal {
    sender: String,
    receiver: String,
    level: SignalLevel,
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.sender, self.level, self.receiver,)
    }
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        states: HashMap<String, SignalLevel>,
    },
    Output,
    Rx,
}

#[derive(Debug)]
struct Module {
    typ: ModuleType,
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
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
            '%' => (ModuleType::FlipFlop { on: false }, &lhs[1..]),
            '&' => (
                ModuleType::Conjunction {
                    states: HashMap::new(),
                },
                &lhs[1..],
            ),
            _ => (ModuleType::Broadcaster, lhs),
        };
        let name = name.to_owned();
        let inputs = Vec::new();
        let outputs = rhs.split(", ").map(|s| s.to_owned()).collect_vec();
        Ok(Module {
            typ,
            name,
            inputs,
            outputs,
        })
    }
}

// Fills in the input fields as the inverse of the output fields.
fn initialize_inputs(modules: &mut HashMap<String, Module>) {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for (_, module) in modules.iter() {
        for output in module.outputs.iter() {
            if !index.contains_key(output) {
                index.insert(output.clone(), Vec::new());
            }
            let v = index.get_mut(output).expect("just checked");
            v.push(module.name.clone());
        }
    }

    for (_, module) in modules.iter_mut() {
        if let Some(inputs) = index.get(&module.name) {
            for input in inputs {
                module.inputs.push(input.clone());
            }
        }
    }
}

fn initialize_conjunctions(modules: &mut HashMap<String, Module>) {
    for (_, module) in modules.iter_mut() {
        if let ModuleType::Conjunction { states } = &mut module.typ {
            for input in module.inputs.iter() {
                states.insert(input.clone(), SignalLevel::Low);
            }
        }
    }
}

fn read_input(path: &str) -> Result<HashMap<String, Module>> {
    let mut map = HashMap::new();
    let modules: Vec<Module> = read_lines(path)?.parse_all()?;
    for module in modules {
        map.insert(module.name.clone(), module);
    }
    map.insert(
        "output".to_owned(),
        Module {
            typ: ModuleType::Output,
            name: "output".to_owned(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        },
    );
    map.insert(
        "rx".to_owned(),
        Module {
            typ: ModuleType::Rx,
            name: "rx".to_owned(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        },
    );
    initialize_inputs(&mut map);
    initialize_conjunctions(&mut map);
    Ok(map)
}

fn run_once(
    iteration: u64,
    modules: &mut HashMap<String, Module>,
    debug: bool,
) -> Result<(u64, u64, bool)> {
    let mut low = 0;
    let mut high = 0;
    let mut rx = false;
    let mut q = VecDeque::new();
    q.push_back(Signal {
        sender: "button".to_owned(),
        receiver: "broadcaster".to_owned(),
        level: SignalLevel::Low,
    });
    while let Some(signal) = q.pop_front() {
        // This prints out the useful signals for part 2.
        if signal.receiver == "zh" {
            if let SignalLevel::High = signal.level {
                println!("{} {}", signal.sender, iteration);
            }
        }
        if debug {
            println!("{}", signal);
        }
        match &signal.level {
            SignalLevel::Low => low += 1,
            SignalLevel::High => high += 1,
        }
        let module = modules
            .get_mut(&signal.receiver)
            .context(format!("output module {} not in input", signal.receiver))?;
        let output_level = match &mut module.typ {
            ModuleType::FlipFlop { on } => {
                if let SignalLevel::Low = signal.level {
                    *on = !*on;
                    if *on {
                        SignalLevel::High
                    } else {
                        SignalLevel::Low
                    }
                } else {
                    continue;
                }
            }
            ModuleType::Conjunction { states } => {
                states.insert(signal.sender.clone(), signal.level);
                let mut all_high = true;
                for (_, level) in states.iter() {
                    if let SignalLevel::Low = level {
                        all_high = false;
                        break;
                    }
                }
                if all_high {
                    SignalLevel::Low
                } else {
                    SignalLevel::High
                }
            }
            ModuleType::Broadcaster => signal.level,
            ModuleType::Output => {
                continue;
            }
            ModuleType::Rx => {
                if let SignalLevel::Low = signal.level {
                    rx = true;
                }
                continue;
            }
        };
        for output in module.outputs.iter() {
            q.push_back(Signal {
                sender: signal.receiver.clone(),
                receiver: output.clone(),
                level: output_level.clone(),
            });
        }
    }
    Ok((low, high, rx))
}

fn part1(modules: &mut HashMap<String, Module>, debug: bool) -> Result<u64> {
    let mut low = 0;
    let mut high = 0;
    for i in 0..1000 {
        let (l, h, _) = run_once(i, modules, debug)?;
        low += l;
        high += h;
    }
    Ok(low * high)
}

// This never finishes, but causes us to output the data needed for part 2.
// Looking at the input data, zh has to send low.
// Each of zh's inputs is an independent graph.
// zh's inputs occur at these offsets:
// bh 3761
// ns 3767
// dl 3779
// vd 3881
// They have an lcm of 207787533680413.
fn part2(modules: &mut HashMap<String, Module>, debug: bool) -> Result<u64> {
    let mut i = 0;
    loop {
        i += 1;
        let (_, _, rx) = run_once(i, modules, debug)?;
        if rx {
            return Ok(i);
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    part2: bool,
}

fn process(args: &Args) -> Result<()> {
    let mut modules = read_input(args.input.as_str())?;
    if args.debug {
        for (_, module) in modules.iter() {
            println!("{:?}", module);
        }
        println!("");
    }

    let ans1 = part1(&mut modules, args.debug)?;
    println!("ans1 = {}", ans1);

    if args.part2 {
        let mut modules = read_input(args.input.as_str())?;
        let ans2 = part2(&mut modules, args.debug)?;
        println!("ans2 = {}", ans2);
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
