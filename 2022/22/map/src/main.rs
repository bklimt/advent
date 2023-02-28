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

#[derive(Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn_right(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }

    fn turn_left(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Right => Orientation::Up,
        }
    }

    fn score(&self) -> usize {
        match self {
            Orientation::Up => 3,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Right => 0,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Orientation::Up => "^".to_string(),
            Orientation::Down => "v".to_string(),
            Orientation::Left => "<".to_string(),
            Orientation::Right => ">".to_string(),
        }
    }
}

struct MapRow {
    offset: usize,
    data: Vec<bool>,
}

impl MapRow {
    fn print(&self, pos: Option<(usize, &Orientation)>) {
        for _ in 0..self.offset {
            print!(" ");
        }
        for (x, b) in self.data.iter().enumerate() {
            if let Some((px, d)) = pos {
                if px == x + self.offset {
                    print!("{}", d.to_string());
                    continue;
                }
            }
            print!("{}", if *b { "#" } else { "." });
        }
    }
}

struct Map {
    rows: Vec<MapRow>,
}

enum Face {
    Top,
    Back,
    Left,
    Front,
    Down,
    Right,
}

impl Map {
    fn initial_pos(&self) -> (usize, usize) {
        (self.rows[0].offset, 0)
    }

    fn face(&self, pos: (usize, usize)) -> Face {
        let (x, y) = pos;
        let face_size = self.rows.len() / 3;
        let latitude = y / face_size;
        let longitude = x / face_size;

        /*
         *   T
         * BLF
         *   DR
         */
        match (latitude, longitude) {
            (0, 2) => Face::Top,
            (1, 0) => Face::Back,
            (1, 1) => Face::Left,
            (1, 2) => Face::Front,
            (2, 2) => Face::Down,
            (2, 3) => Face::Right,
            _ => panic!("bad face: {}, {}", latitude, longitude),
        }
    }

    fn print(&self, pos: (usize, usize), dir: &Orientation) {
        let (px, py) = pos;
        for (y, row) in self.rows.iter().enumerate() {
            if py == y {
                row.print(Some((px, dir)));
            } else {
                row.print(None);
            }
            println!("");
        }
    }

    fn walk_right(&self, pos: (usize, usize), dist: i64) -> (usize, usize) {
        let mut pos = pos;
        for _ in 0..dist {
            let row = self.rows.get(pos.1).unwrap();
            let new_x = if pos.0 == row.offset + row.data.len() - 1 {
                row.offset
            } else {
                pos.0 + 1
            };
            if row.data[new_x - row.offset] {
                break;
            }
            pos = (new_x, pos.1);
        }
        pos
    }

    fn walk_left(&self, pos: (usize, usize), dist: i64) -> (usize, usize) {
        let mut pos = pos;
        for _ in 0..dist {
            let row = self.rows.get(pos.1).unwrap();
            let new_x = if pos.0 == row.offset {
                (row.offset + row.data.len()) - 1
            } else {
                pos.0 - 1
            };
            if row.data[new_x - row.offset] {
                break;
            }
            pos = (new_x, pos.1);
        }
        pos
    }

    fn walk_up(&self, pos: (usize, usize), dist: i64) -> (usize, usize) {
        let mut pos = pos;
        for _ in 0..dist {
            let mut new_y = if pos.1 == 0 {
                self.rows.len() - 1
            } else {
                pos.1 - 1
            };
            // Handle wrapping.
            loop {
                let new_row = &self.rows[new_y];
                if pos.0 >= new_row.offset && pos.0 < new_row.offset + new_row.data.len() {
                    break;
                }
                new_y = if new_y == 0 {
                    self.rows.len() - 1
                } else {
                    new_y - 1
                };
            }
            // Check for a wall.
            let new_row = &self.rows[new_y];
            if new_row.data[pos.0 - new_row.offset] {
                break;
            }
            pos = (pos.0, new_y);
        }
        pos
    }

    fn walk_down(&self, pos: (usize, usize), dist: i64) -> (usize, usize) {
        let mut pos = pos;
        for _ in 0..dist {
            let mut new_y = if pos.1 == self.rows.len() - 1 {
                0
            } else {
                pos.1 + 1
            };
            // Handle wrapping.
            loop {
                let new_row = &self.rows[new_y];
                if pos.0 >= new_row.offset && pos.0 < new_row.offset + new_row.data.len() {
                    break;
                }
                new_y = if new_y == self.rows.len() - 1 {
                    0
                } else {
                    new_y + 1
                };
            }
            // Check for a wall.
            let new_row = &self.rows[new_y];
            if new_row.data[pos.0 - new_row.offset] {
                break;
            }
            pos = (pos.0, new_y);
        }
        pos
    }

    fn walk(&self, pos: (usize, usize), dir: &Orientation, dist: i64) -> (usize, usize) {
        match dir {
            Orientation::Up => self.walk_up(pos, dist),
            Orientation::Down => self.walk_down(pos, dist),
            Orientation::Left => self.walk_left(pos, dist),
            Orientation::Right => self.walk_right(pos, dist),
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

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let (map, instructions) = read_input(&args.input, args.debug)?;

    let mut pos = map.initial_pos();
    let mut dir = Orientation::Right;
    if args.debug {
        map.print(pos, &dir);
        println!("{:?}", instructions);
    }

    for step in instructions {
        if args.debug {
            println!("Facing {:?} @ ({}, {})", dir, pos.0, pos.1);
            println!("Step: {:?}", step);
        }
        match step {
            Instruction::Forward(dist) => {
                pos = map.walk(pos, &dir, dist);
            }
            Instruction::Right => {
                dir = dir.turn_right();
            }
            Instruction::Left => {
                dir = dir.turn_left();
            }
        }
        if args.debug {
            map.print(pos, &dir);
            println!("");
        }
    }
    if args.debug {
        println!("Facing {:?} @ ({}, {})", dir, pos.0, pos.1);
    }

    let ans = 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir.score();
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
