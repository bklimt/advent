use anyhow::{anyhow, Context, Result};
use clap::Parser;
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

struct MapRow {
    offset: usize,
    data: Vec<bool>,
}

impl MapRow {
    fn get(&self, i: usize) -> Option<bool> {
        return if i < self.offset {
            None
        } else if (i - self.offset) >= self.data.len() {
            None
        } else {
            Some(self.data[i - self.offset])
        };
    }

    fn set(&mut self, i: usize, val: bool) -> Result<()> {
        if i < self.offset || (i - self.offset) >= self.data.len() {
            return Err(anyhow!(
                "invalid offset: {} in row from {} of len {}",
                i,
                self.offset,
                self.data.len()
            ));
        }
        self.data[i - self.offset] = val;
        Ok(())
    }

    fn print(&self) {
        for _ in 0..self.offset {
            print!(" ");
        }
        for b in self.data.iter() {
            print!("{}", if *b { "#" } else { "." });
        }
    }
}

struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn height(&self) -> usize {
        self.rows.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        return if y >= self.rows.len() {
            None
        } else {
            self.rows[y].get(x)
        };
    }

    fn set(&mut self, x: usize, y: usize, val: bool) -> Result<()> {
        return if y >= self.rows.len() {
            Err(anyhow!("invalid row: {} out of {}", y, self.rows.len()))
        } else {
            self.rows[y].set(x, val)
        };
    }

    fn print(&self) {
        for row in self.rows.iter() {
            row.print();
            println!("");
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i64),
    Right,
    Left,
}

impl Instruction {
    fn parse(s: &str) -> Result<Option<(Instruction, &str)>> {
        return if s == "" {
            Ok(None)
        } else if let Some(i) = s.find(|c| c == 'R' || c == 'L') {
            if i == 0 {
                let (d, s) = s.split_at(1);
                let instruction = match d {
                    "R" => Instruction::Right,
                    "L" => Instruction::Left,
                    _ => panic!("invalid instruction: {}", d),
                };
                Ok(Some((instruction, s)))
            } else {
                let (sn, s) = s.split_at(i);
                let n = sn.parse::<i64>()?;
                Ok(Some((Instruction::Forward(n), s)))
            }
        } else {
            Ok(Some((Instruction::Forward(s.parse::<i64>()?), "")))
        };
    }

    fn parse_list(s: &str) -> Result<Vec<Instruction>> {
        let mut v = Vec::new();
        let mut ss = s;
        while let Some((instruction, s)) = Instruction::parse(ss)? {
            v.push(instruction);
            ss = s;
        }
        Ok(v)
    }
}

fn read_input(path: &str, _debug: bool) -> Result<(Map, Vec<Instruction>)> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut rows = Vec::new();
    let mut instructions = None;
    let mut in_map = true;
    loop {
        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim_end();

        if line == "" {
            in_map = false;
            if n == 0 {
                break;
            }
            continue;
        }

        if in_map {
            let mut data = Vec::new();
            let mut offset = 0;
            for c in line.chars() {
                match c {
                    ' ' => {
                        offset = offset + 1;
                    }
                    '.' => {
                        data.push(false);
                    }
                    '#' => {
                        data.push(true);
                    }
                    _ => {
                        return Err(anyhow!("invalid character in map: {}", c));
                    }
                }
            }
            rows.push(MapRow { offset, data });
        } else {
            if instructions.is_some() {
                return Err(anyhow!("duplicate instructions: {}", line));
            }
            instructions = Some(Instruction::parse_list(line)?);
        }
    }
    let m = Map { rows };
    return if let Some(instructions) = instructions {
        Ok((m, instructions))
    } else {
        Err(anyhow!("missing instructions"))
    };
}

fn turn_right(p: (i64, i64)) -> (i64, i64) {
    // (1, 0) -> (0, 1)
    // (0, 1) -> (-1, 0)
    // (-1, 0) -> (0, -1)
    // (0, -1) -> (1, 0)
    return if p.1 == 0 { (0, p.0) } else { (-1 * p.0, 0) };
}

fn turn_left(p: (i64, i64)) -> (i64, i64) {
    return if p.0 == 0 { (p.1, 0) } else { (0, -1 * p.0) };
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let (map, instructions) = read_input(&args.input, args.debug)?;
    if args.debug {
        map.print();
        println!("{:?}", instructions);
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
