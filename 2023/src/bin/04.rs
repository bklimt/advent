use advent::common::{read_lines, StrIterator};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::option::Option;
use std::str::FromStr;

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

impl FromStr for Card {
    type Err = anyhow::Error;

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

        let winners_vec = winners_str.split_whitespace().parse_all()?;
        let chosen = chosen_str.split_whitespace().parse_all()?;

        let mut winners = HashSet::new();
        for winner in winners_vec {
            winners.insert(winner);
        }

        Ok(Card { winners, chosen })
    }
}

impl Card {
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
    let mut cards = Vec::new();
    for line in read_lines(path)? {
        cards.push(line.parse()?);
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
