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

    #[arg(long)]
    part2: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    /*
                11F1
                C11A
                1111
                1111
        2F223C334444
        D2223333444B
        222233334444
        2G2233E34444
                555566B6
                E5556666
                5555666A
                55G566D6
    */

    fn wrap_right(&self, pos: (usize, usize)) -> Option<((usize, usize), Orientation)> {
        let face_size = self.rows.len() / 3;
        let (x, y) = pos;
        match self.face(pos) {
            // A
            Face::Top => {
                if x == face_size * 3 - 1 {
                    Some((
                        ((face_size * 4) - 1, self.rows.len() - (y + 1)),
                        Orientation::Left,
                    ))
                } else {
                    None
                }
            }
            // B
            Face::Front => {
                if x == face_size * 3 - 1 {
                    Some((
                        ((face_size * 4) - ((y - face_size) + 1), face_size * 2),
                        Orientation::Down,
                    ))
                } else {
                    None
                }
            }
            // A
            Face::Right => {
                if x == face_size * 4 - 1 {
                    Some((
                        ((face_size * 3) - 1, (self.rows.len() - 1) - y),
                        Orientation::Left,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn wrap_left(&self, pos: (usize, usize)) -> Option<((usize, usize), Orientation)> {
        let face_size = self.rows.len() / 3;
        let (x, y) = pos;
        match self.face(pos) {
            // C
            Face::Top => {
                if x == face_size * 2 {
                    Some(((face_size + y, face_size), Orientation::Down))
                } else {
                    None
                }
            }
            // D
            Face::Back => {
                if x == 0 {
                    Some((
                        (
                            ((face_size * 4) - 1) - (y - face_size),
                            (self.rows.len()) - 1,
                        ),
                        Orientation::Up,
                    ))
                } else {
                    None
                }
            }
            // E
            Face::Down => {
                if x == face_size * 2 {
                    Some((
                        (face_size + ((face_size * 3) - (y + 1)), face_size * 2 - 1),
                        Orientation::Up,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn wrap_up(&self, pos: (usize, usize)) -> Option<((usize, usize), Orientation)> {
        let face_size = self.rows.len() / 3;
        let (x, y) = pos;
        match self.face(pos) {
            // F
            Face::Top => {
                if y == 0 {
                    Some((((face_size * 3) - (x + 1), face_size), Orientation::Down))
                } else {
                    None
                }
            }
            // F
            Face::Back => {
                if y == face_size {
                    Some((((face_size * 3) - (x + 1), 0), Orientation::Down))
                } else {
                    None
                }
            }
            // C
            Face::Left => {
                if y == face_size {
                    Some(((face_size * 2, x - face_size), Orientation::Right))
                } else {
                    None
                }
            }
            // B
            Face::Right => {
                if y == face_size * 2 {
                    Some((
                        (face_size * 3 - 1, face_size + (face_size * 4) - (x + 1)),
                        Orientation::Left,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn wrap_down(&self, pos: (usize, usize)) -> Option<((usize, usize), Orientation)> {
        let face_size = self.rows.len() / 3;
        let (x, y) = pos;
        match self.face(pos) {
            // G
            Face::Back => {
                if y == face_size * 2 - 1 {
                    Some((
                        ((face_size * 3) - (x + 1), self.rows.len() - 1),
                        Orientation::Up,
                    ))
                } else {
                    None
                }
            }
            // E
            Face::Left => {
                if y == face_size * 2 - 1 {
                    Some((
                        (face_size * 2, ((face_size * 2) + (face_size * 2) - (x + 1))),
                        Orientation::Right,
                    ))
                } else {
                    None
                }
            }
            // G
            Face::Down => {
                if y == self.rows.len() - 1 {
                    Some((
                        ((face_size * 3) - (x + 1), face_size * 2 - 1),
                        Orientation::Up,
                    ))
                } else {
                    None
                }
            }
            // D
            Face::Right => {
                if y == self.rows.len() - 1 {
                    Some((
                        (0, face_size + (face_size * 4) - (x + 1)),
                        Orientation::Right,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn wrap(
        &self,
        pos: (usize, usize),
        dir: &Orientation,
    ) -> Option<((usize, usize), Orientation)> {
        match dir {
            Orientation::Up => self.wrap_up(pos),
            Orientation::Down => self.wrap_down(pos),
            Orientation::Left => self.wrap_left(pos),
            Orientation::Right => self.wrap_right(pos),
        }
    }

    fn walk2(
        &self,
        pos: (usize, usize),
        dir: &Orientation,
        dist: i64,
    ) -> ((usize, usize), Orientation) {
        let mut pos = pos;
        let mut dir = *dir;
        for _ in 0..dist {
            let (new_pos, new_dir) = self.wrap(pos, &dir).unwrap_or_else(|| match dir {
                Orientation::Up => ((pos.0, pos.1 - 1), Orientation::Up),
                Orientation::Down => ((pos.0, pos.1 + 1), Orientation::Down),
                Orientation::Left => ((pos.0 - 1, pos.1), Orientation::Left),
                Orientation::Right => ((pos.0 + 1, pos.1), Orientation::Right),
            });
            let new_row = self.rows.get(new_pos.1).unwrap();
            if new_row.data[new_pos.0 - new_row.offset] {
                break;
            }
            pos = new_pos;
            dir = new_dir;
        }
        (pos, dir)
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

    fn walk1(
        &self,
        pos: (usize, usize),
        dir: &Orientation,
        dist: i64,
    ) -> ((usize, usize), Orientation) {
        (
            match dir {
                Orientation::Up => self.walk_up(pos, dist),
                Orientation::Down => self.walk_down(pos, dist),
                Orientation::Left => self.walk_left(pos, dist),
                Orientation::Right => self.walk_right(pos, dist),
            },
            *dir,
        )
    }

    fn walk(
        &self,
        pos: (usize, usize),
        dir: &Orientation,
        dist: i64,
        part2: bool,
    ) -> ((usize, usize), Orientation) {
        if part2 {
            self.walk2(pos, dir, dist)
        } else {
            self.walk1(pos, dir, dist)
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
    if args.debug {
        println!("reading input...");
    }
    let (map, instructions) = read_input(&args.input, args.debug)?;

    let mut pos = map.initial_pos();
    let mut dir = Orientation::Right;
    if args.debug {
        map.print(pos, &dir);
        println!("{:?}", instructions);
        println!("rows = {}", map.rows.len());
        println!("face_size = {}", map.rows.len() / 3);
    }

    for step in instructions {
        if args.debug {
            println!("Facing {:?} @ ({}, {})", dir, pos.0, pos.1);
            println!("Step: {:?}", step);
        }
        match step {
            Instruction::Forward(dist) => {
                (pos, dir) = map.walk(pos, &dir, dist, args.part2);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap() {
        let (m, _) = read_input("./empty4x4.txt", true).unwrap();
        assert_eq!(m.rows.len(), 12);

        let cases: Vec<(usize, usize, Orientation, usize, usize, Orientation)> = vec![
            (8, 0, Orientation::Right, 9, 0, Orientation::Right),
            // A
            (11, 0, Orientation::Right, 15, 11, Orientation::Left),
            (15, 11, Orientation::Right, 11, 0, Orientation::Left),
            // B
            (11, 5, Orientation::Right, 14, 8, Orientation::Down),
            (14, 8, Orientation::Up, 11, 5, Orientation::Left),
            // C
            (8, 1, Orientation::Left, 5, 4, Orientation::Down),
            (5, 4, Orientation::Up, 8, 1, Orientation::Right),
            // D
            (0, 5, Orientation::Left, 14, 11, Orientation::Up),
            (14, 11, Orientation::Down, 0, 5, Orientation::Right),
            // E
            (8, 9, Orientation::Left, 6, 7, Orientation::Up),
            (6, 7, Orientation::Down, 8, 9, Orientation::Right),
            // F
            (1, 4, Orientation::Up, 10, 0, Orientation::Down),
            (10, 0, Orientation::Up, 1, 4, Orientation::Down),
            // G
            (1, 7, Orientation::Down, 10, 11, Orientation::Up),
            (10, 11, Orientation::Down, 1, 7, Orientation::Up),
        ];
        for (i, case) in cases.iter().enumerate() {
            let (start_x, start_y, start_dir, exp_x, exp_y, exp_dir) = case;
            let ((act_x, act_y), act_dir) = m.walk((*start_x, *start_y), start_dir, 1, true);
            println!(
                "case {}: walking from ({}, {}) {:?}: = ({}, {}) {:?}; want ({} {}) {:?};",
                i, start_x, start_y, &start_dir, act_x, act_y, &act_dir, exp_x, exp_y, &exp_dir,
            );
            assert_eq!(act_x, *exp_x);
            assert_eq!(act_y, *exp_y);
            assert_eq!(act_dir, *exp_dir);
        }
    }
}
