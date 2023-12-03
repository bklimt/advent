use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashSet;
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

// Where a number is, with the line and range [start, end).
#[derive(Debug)]
struct NumLoc {
    row: i32,
    col_start: i32,
    col_end: i32,
    num: i32,
}

// Where a part is.
#[derive(Debug, Hash, Eq, PartialEq)]
struct PartLoc {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Board {
    nums: Vec<NumLoc>,
    parts: HashSet<PartLoc>,
}

impl Board {
    fn new() -> Self {
        Board {
            nums: Vec::new(),
            parts: HashSet::new(),
        }
    }

    fn add_nums_and_parts(&mut self, row: i32, line: &str) -> Result<()> {
        let mut current_num: Option<NumLoc> = None;

        for (col, c) in line.char_indices() {
            match c {
                '0'..='9' => {
                    if current_num.is_none() {
                        current_num = Some(NumLoc {
                            row: row,
                            col_start: col as i32,
                            col_end: (col + 1) as i32,
                            num: 0,
                        });
                    }
                    let n = current_num.as_ref().unwrap().num;
                    let d = c.to_digit(10).context(format!("invalid digit: {}", c))? as i32;
                    current_num.as_mut().unwrap().num = n * 10 + d;
                    current_num.as_mut().unwrap().col_end = (col + 1) as i32;
                }
                _ => {
                    if current_num.is_some() {
                        self.nums.push(current_num.take().unwrap());
                    }
                    if c != '.' {
                        self.parts.insert(PartLoc {
                            row: row,
                            col: col as i32,
                        });
                    }
                }
            }
        }

        if current_num.is_some() {
            self.nums.push(current_num.take().unwrap());
        }
        Ok(())
    }

    fn is_num_near_part(&self, num: &NumLoc) -> bool {
        for row in num.row - 1..=num.row + 1 {
            for col in num.col_start - 1..num.col_end + 1 {
                if self.parts.contains(&PartLoc { row, col }) {
                    return true;
                }
            }
        }
        false
    }

    fn part1(&self) -> i32 {
        let mut ans = 0;
        for num in self.nums.iter() {
            if self.is_num_near_part(&num) {
                ans += num.num;
            }
        }
        ans
    }
}

fn read_input(path: &str, _debug: bool) -> Result<Board> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut board = Board::new();
    let mut row: i32 = 0;
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

        board.add_nums_and_parts(row, line)?;
        row += 1;
    }
    Ok(board)
}

fn process(args: &Args) -> Result<()> {
    let board = read_input(&args.input, args.debug)?;
    println!("ans1: {}", board.part1());
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
