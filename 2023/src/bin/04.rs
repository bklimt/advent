use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet};
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
struct Card {
    winners: HashSet<i32>,
    chosen: Vec<i32>,
}

fn parse_num_list(line: &str) -> Result<Vec<i32>> {
    let mut v = Vec::new();
    for part in line.split_whitespace() {
        let part = part.trim();
        v.push(part.parse()?);
    }
    Ok(v)
}

impl Card {
    fn from_str(line: &str) -> Result<Card> {
        if !line.starts_with("Card ") {
            return Err(anyhow!("invalid line: {:?}", line));
        }
        let line = &line[5..];

        let colon_pos = line
            .find(':')
            .context(format!("missing colon: {:?}", line))?;
        let (_, line) = line.split_at(colon_pos);
        let line = &line[1..];

        let pipe_pos = line
            .find('|')
            .context(format!("missing pipe: {:?}", line))?;
        let (winners_str, chosen_str) = line.split_at(pipe_pos);
        let chosen_str = &chosen_str[1..];

        let winners_vec = parse_num_list(winners_str)?;
        let chosen = parse_num_list(chosen_str)?;

        let mut winners = HashSet::new();
        for winner in winners_vec {
            winners.insert(winner);
        }

        Ok(Card { winners, chosen })
    }

    fn count(&self) -> i32 {
        let mut count = 0;
        for n in self.chosen.iter() {
            if self.winners.contains(n) {
                count += 1;
            }
        }
        count
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for n in self.chosen.iter() {
            if self.winners.contains(n) {
                score = match score {
                    0 => 1,
                    _ => score * 2,
                };
            }
        }
        score
    }
}

fn score_cards(v: &Vec<Card>) -> i32 {
    let mut score = 0;
    for c in v.iter() {
        score += c.score();
    }
    score
}

fn do_part2(v: &Vec<Card>, debug: bool) -> Result<i32> {
    let mut score = 0;
    let mut score_map: HashMap<usize, i32> = HashMap::new();
    for i in (0..v.len()).rev() {
        let mut count: i32 = 1;
        let more = v[i].count() as usize;
        for m in 1..=more {
            let j = i + m;
            if j >= v.len() {
                return Err(anyhow!("card {} references out-of-bounds card {}", i, j));
            }
            let additional = score_map
                .get(&j)
                .context(format!("missing card: {} from {}", j, i))?;
            count += *additional;
        }
        score_map.insert(i, count);
        score += count;
        if debug {
            println!("{}: {}", i, count);
        }
    }
    Ok(score)
}

fn read_input(path: &str, _debug: bool) -> Result<Vec<Card>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut cards = Vec::new();
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

        cards.push(Card::from_str(line)?);
    }
    Ok(cards)
}

fn process(args: &Args) -> Result<()> {
    let cards = read_input(&args.input, args.debug)?;
    println!("ans1: {}", score_cards(&cards));
    println!("ans2: {}", do_part2(&cards, args.debug)?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
