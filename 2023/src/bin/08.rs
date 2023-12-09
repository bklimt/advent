use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

#[derive(Debug)]
enum Direction {
    Left = 0,
    Right,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

fn read_input(path: &str, _debug: bool) -> Result<Input> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut directions: Option<String> = None;
    let mut map = HashMap::new();
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

        if directions.is_none() {
            directions = Some(line.to_owned());
        } else {
            // AAA = (BBB, CCC)
            if line.len() != 16 {
                return Err(anyhow!("invalid map line: {}", line));
            }
            let start = line[..3].to_owned();
            let left = line[7..10].to_owned();
            let right = line[12..15].to_owned();
            map.insert(start, (left, right));
        }
    }
    let directions = directions.context("missing directions")?;
    let directions: Result<Vec<Direction>> = directions
        .chars()
        .map(|c| match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("invalid direction: {}", c)),
        })
        .collect();
    let directions = directions?;

    Ok(Input { directions, map })
}

fn traverse(input: &Input) -> Result<u64> {
    let mut total = 0;
    let mut i = 0;
    let mut current: &String = &"AAA".to_owned();
    while current != "ZZZ" {
        let (left, right) = input
            .map
            .get(current)
            .context(format!("unknown position: {}", current))?;

        current = match input.directions[i] {
            Direction::Left => left,
            Direction::Right => right,
        };

        total += 1;
        i = (i + 1) % input.directions.len();
    }
    Ok(total)
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(&args.input, args.debug)?;
    let ans = traverse(&input)?;
    println!("ans1: {}", ans);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
