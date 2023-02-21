use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::{EitherOrBoth::*, Itertools};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,
}

enum Token {
    EndOfInput,
    OpenBracket,
    CloseBracket,
    Comma,
    Number(u32),
}

fn read_number(chars: &mut Peekable<Chars>) -> Result<Token> {
    if chars.peek().is_none() {
        return Err(anyhow!("expected digit, got end of string"));
    }

    let mut num = 0;
    loop {
        match chars.peek() {
            None | Some(']' | ',') => {
                return Ok(Token::Number(num));
            }
            Some(c) => {
                if !c.is_digit(10) {
                    return Err(anyhow!("expected digit, got {}", c));
                }
                num = (num * 10) + c.to_digit(10).unwrap();
            }
        }
        chars.next();
    }
}

fn read_token(chars: &mut Peekable<Chars>) -> Result<Token> {
    match chars.peek() {
        None => Ok(Token::EndOfInput),
        Some('[') => {
            chars.next();
            Ok(Token::OpenBracket)
        }
        Some(']') => {
            chars.next();
            Ok(Token::CloseBracket)
        }
        Some(',') => {
            chars.next();
            Ok(Token::Comma)
        }
        _ => read_number(chars),
    }
}

#[derive(Debug)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

fn read_list(chars: &mut Peekable<Chars>) -> Result<Packet> {
    let mut v = Vec::new();
    loop {
        match read_token(chars)? {
            Token::CloseBracket => {
                return Ok(Packet::List(v));
            }
            Token::Number(n) => {
                v.push(Packet::Number(n));
            }
            Token::OpenBracket => {
                v.push(read_list(chars)?);
            }
            Token::Comma => {}
            Token::EndOfInput => {
                return Err(anyhow!("expected list, got end of input"));
            }
        }
    }
}

fn read_packet(s: &str) -> Result<Packet> {
    let mut chars = s.chars().peekable();
    match read_token(&mut chars)? {
        Token::OpenBracket => read_list(&mut chars),
        _ => Err(anyhow!("expected [")),
    }
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    match left {
        Packet::Number(l) => match right {
            Packet::Number(r) => {
                if l < r {
                    Ordering::Less
                } else if r < l {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            Packet::List(_) => {
                let mut l2 = Vec::new();
                l2.push(Packet::Number(*l));
                compare(&Packet::List(l2), right)
            }
        },
        Packet::List(l) => match right {
            Packet::Number(r) => {
                let mut r2 = Vec::new();
                r2.push(Packet::Number(*r));
                compare(left, &Packet::List(r2))
            }
            Packet::List(r) => {
                for pair in l.iter().zip_longest(r.iter()) {
                    match pair {
                        Both(l, r) => match compare(l, r) {
                            Ordering::Equal => {}
                            Ordering::Less => {
                                return Ordering::Less;
                            }
                            Ordering::Greater => {
                                return Ordering::Greater;
                            }
                        },
                        Left(_) => {
                            return Ordering::Greater;
                        }
                        Right(_) => {
                            return Ordering::Less;
                        }
                    }
                }
                Ordering::Equal
            }
        },
    }
}

fn read_input(path: &str) -> Result<()> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut index = 1;
    let mut last_packet = Packet::Number(0);
    let mut sum = 0;
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

        println!("line: {}", line);

        let packet = read_packet(line)?;
        // println!("packet: {:?}", packet);

        if index % 2 == 0 {
            let ans = compare(&last_packet, &packet);
            println!("compare: {:?}", ans);
            println!("");
            match ans {
                Ordering::Less => {
                    sum = sum + (index / 2);
                }
                _ => {}
            }
        }
        index = index + 1;
        last_packet = packet;
    }
    println!("sum: {}", sum);
    Ok(())
}

fn process(path: &str, _part2: bool) -> Result<()> {
    let _ = read_input(path)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
