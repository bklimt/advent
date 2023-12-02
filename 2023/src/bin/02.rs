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

    #[arg(long)]
    part2: bool,
}

#[derive(Debug)]
struct Game {
    number: u32,
    draws: Vec<HashMap<String, u32>>,
}

impl Game {
    fn is_possible(&self, totals: &HashMap<String, u32>) -> bool {
        for draw in self.draws.iter() {
            for (color, count) in draw.iter() {
                let total = totals.get(color).unwrap_or(&0);
                if *count > *total {
                    return false;
                }
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let mut max_counts: HashMap<String, u32> = HashMap::new();
        for draw in self.draws.iter() {
            for (color, count) in draw.iter() {
                let max_count = max_counts.get(color).unwrap_or(&0);
                if *count > *max_count {
                    max_counts.insert(color.into(), *count);
                }
            }
        }
        let mut p = 1;
        for (_, count) in max_counts.iter() {
            if *count > 0 {
                p *= *count;
            }
        }
        p
    }
}

fn read_game(line: &str) -> Result<Game> {
    if !line.starts_with("Game ") {
        return Err(anyhow!("invalid line: {}", line));
    }
    let line = &line[5..];

    let (num_str, line) = line
        .split_once(": ")
        .context(format!("invalid game: {}", line))?;

    let num = num_str.parse::<u32>()?;
    let mut draws = Vec::new();

    let draw_parts = line.split("; ");
    for draw_part in draw_parts {
        let mut draw: HashMap<String, u32> = HashMap::new();
        let color_parts = draw_part.split(", ");
        for color_part in color_parts {
            let (count_str, color) = color_part
                .split_once(" ")
                .context(format!("invalid count: {}", color_part))?;

            let count = count_str.parse::<u32>()?;

            draw.insert(color.into(), count);
        }
        draws.push(draw);
    }

    Ok(Game {
        number: num,
        draws: draws,
    })
}

fn read_input(path: &str, _debug: bool) -> Result<Vec<Game>> {
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

        let game = read_game(line)?;
        v.push(game);
    }
    Ok(v)
}

fn process(args: &Args) -> Result<()> {
    let mut totals: HashMap<String, u32> = HashMap::new();
    totals.insert("red".into(), 12);
    totals.insert("green".into(), 13);
    totals.insert("blue".into(), 14);

    let mut ans1: u64 = 0;
    let mut ans2: u64 = 0;

    let games = read_input(&args.input, args.debug)?;
    for game in games {
        if args.debug {
            println!("game: {:?}", game);
        }
        let power = game.power();
        ans2 += power as u64;
        if game.is_possible(&totals) {
            ans1 += game.number as u64;
            if args.debug {
                println!("possible!");
            }
        } else {
            if args.debug {
                println!("impossible!");
            }
        }
        if args.debug {
            println!("power: {}", power);
        }
    }

    println!("ans1: {}", ans1);
    println!("ans2: {}", ans2);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
