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
    part2: bool,

    #[arg(long)]
    debug: bool,
}

#[derive(Copy, Clone, Debug)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn compute_handtype(values: [i32; 5]) -> Result<HandType> {
    // Count how often each card appears in the hand.
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut jokers = 0;
    for n in values.iter() {
        if *n == 1 {
            jokers += 1;
        } else {
            counts.insert(*n, counts.get(n).unwrap_or(&0) + 1);
        }
    }
    let mut counts: Vec<i32> = counts.into_values().collect();
    counts.sort();
    counts.reverse();

    let typ = match counts.len() {
        0 => HandType::FiveOfAKind,
        1 => HandType::FiveOfAKind,
        2 => {
            if counts[0] == 4 - jokers {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if counts[0] == 3 - jokers {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => return Err(anyhow!("invalid set size: {}", counts.len())),
    };
    Ok(typ)
}

#[derive(Debug)]
struct Hand {
    _text: String,
    values: [i32; 5],
    typ: HandType,
}

impl Hand {
    fn from_str(text: &str, part2: bool) -> Result<Self> {
        if text.len() != 5 {
            return Err(anyhow!("invalid hand: {}", text));
        }

        let mut values: [i32; 5] = [0; 5];
        for (i, c) in text.chars().enumerate() {
            values[i] = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if part2 {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                '0'..='9' => c.to_digit(10).unwrap() as i32,
                _ => return Err(anyhow!("invalid card char: {}", c)),
            };
        }

        let typ = compute_handtype(values)?;

        let text = text.to_owned();
        Ok(Hand {
            _text: text,
            values,
            typ,
        })
    }
}

#[derive(Debug)]
struct Record {
    hand: Hand,
    bid: i32,
}

impl Record {
    fn from_str(line: &str, part2: bool) -> Result<Self> {
        Ok(Record {
            hand: Hand::from_str(&line[..5], part2)?,
            bid: (&line[6..])
                .parse()
                .context(format!("invalid bid: {}", line))?,
        })
    }
}

fn read_input(path: &str, part2: bool, _debug: bool) -> Result<Vec<Record>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut records = Vec::new();
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

        records.push(Record::from_str(line, part2)?);
    }
    Ok(records)
}

fn process(args: &Args) -> Result<()> {
    let mut records = read_input(&args.input, args.part2, args.debug)?;

    records.sort_by_key(|r: &Record| (r.hand.typ as i32, r.hand.values));

    let mut ans = 0;
    for (i, record) in records.iter().enumerate() {
        println!("{:?}", record);
        let rank = i as i32 + 1;
        ans += record.bid * rank;
    }

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
