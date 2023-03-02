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

#[derive(Debug)]
enum Op {
    Number(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl Op {
    fn parse_binary_op(s: &str, op: char) -> Result<Option<(String, String)>> {
        if let Some(p) = s.find(op) {
            let (s1, s2) = s.split_at(p);
            let s1 = s1.trim();
            let s2 = s2
                .strip_prefix(op)
                .ok_or_else(|| anyhow!("invalid: {}", s2))?
                .trim();
            Ok(Some((s1.to_string(), s2.to_string())))
        } else {
            Ok(None)
        }
    }

    fn from_str(s: &str) -> Result<Op> {
        return if let Some((s1, s2)) = Op::parse_binary_op(s, '+')? {
            Ok(Op::Add(s1.to_string(), s2.to_string()))
        } else if let Some((s1, s2)) = Op::parse_binary_op(s, '-')? {
            Ok(Op::Subtract(s1.to_string(), s2.to_string()))
        } else if let Some((s1, s2)) = Op::parse_binary_op(s, '*')? {
            Ok(Op::Multiply(s1.to_string(), s2.to_string()))
        } else if let Some((s1, s2)) = Op::parse_binary_op(s, '/')? {
            Ok(Op::Divide(s1.to_string(), s2.to_string()))
        } else {
            Ok(Op::Number(
                s.parse::<i64>()
                    .with_context(|| format!("bad number: {}", s))?,
            ))
        };
    }

    fn to_string(&self) -> String {
        match self {
            Op::Number(n) => format!("{}", n),
            Op::Add(a, b) => format!("{} + {}", a, b),
            Op::Subtract(a, b) => format!("{} - {}", a, b),
            Op::Multiply(a, b) => format!("{} * {}", a, b),
            Op::Divide(a, b) => format!("{} / {}", a, b),
        }
    }
}

fn parse_line(s: &str) -> Result<(String, Op)> {
    if let Some(p) = s.find(':') {
        let (s1, s2) = s.split_at(p);
        let s1 = s1.trim();
        let s2 = s2.strip_prefix(':').unwrap().trim();
        Ok((s1.to_string(), Op::from_str(s2)?))
    } else {
        Err(anyhow!("invalid line: {}", s))
    }
}

fn read_input(path: &str, debug: bool) -> Result<HashMap<String, Op>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut m = HashMap::new();
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
        let (monkey, op) = parse_line(line)?;
        if debug {
            println!("{} -> {}", monkey, op.to_string());
        }
        m.insert(monkey, op);
    }
    Ok(m)
}

fn compute(m: &HashMap<String, Op>, s: &str, debug: bool) -> i64 {
    if debug {
        println!("looking up {}", s);
    }
    match m.get(s).unwrap() {
        Op::Number(n) => *n,
        Op::Add(x, y) => compute(m, x.as_str(), debug) + compute(m, y.as_str(), debug),
        Op::Subtract(x, y) => compute(m, x.as_str(), debug) - compute(m, y.as_str(), debug),
        Op::Multiply(x, y) => compute(m, x.as_str(), debug) * compute(m, y.as_str(), debug),
        Op::Divide(x, y) => compute(m, x.as_str(), debug) / compute(m, y.as_str(), debug),
    }
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let m = read_input(&args.input, args.debug)?;
    let ans = compute(&m, "root", args.debug);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(
            matches!(Op::from_str("abcd + efgh").unwrap(), Op::Add(x1, x2) if x1 == "abcd" && x2 == "efgh")
        );
        assert!(
            matches!(Op::from_str("abcd - efgh").unwrap(), Op::Subtract(x1, x2) if x1 == "abcd" && x2 == "efgh")
        );
        assert!(
            matches!(Op::from_str("abcd * efgh").unwrap(), Op::Multiply(x1, x2) if x1 == "abcd" && x2 == "efgh")
        );
        assert!(
            matches!(Op::from_str("abcd / efgh").unwrap(), Op::Divide(x1, x2) if x1 == "abcd" && x2 == "efgh")
        );
        assert!(matches!(Op::from_str("123").unwrap(), Op::Number(x) if x == 123));

        let (monkey, op) = parse_line("abcd: efgh + ijkl").unwrap();
        assert_eq!(monkey, "abcd");
        assert!(matches!(op, Op::Add(x, y) if x == "efgh" && y == "ijkl"));
    }
}
