use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    part2: bool,
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Sand,
    Rock,
}

struct Map {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    tiles: HashMap<(usize, usize), Tile>,
}

impl Map {
    fn new(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> Self {
        Map {
            min_x,
            min_y,
            max_x,
            max_y,
            tiles: HashMap::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles.get(&(x, y)).unwrap_or(&Tile::Empty).clone()
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles.insert((x, y), tile);
    }
}

fn print_map(m: &Map) {
    for y in m.min_y..=m.max_y {
        for x in m.min_x..=m.max_x {
            let t = m.get(x, y);
            print!(
                "{}",
                match t {
                    Tile::Empty => '.',
                    Tile::Sand => 'o',
                    Tile::Rock => '#',
                }
            );
        }
        println!("");
    }
}

fn read_input(path: &str) -> Result<Map> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut data = Vec::new();
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
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
        let mut row = Vec::new();
        for pair in line.split(" -> ") {
            println!("pair: {}", pair);
            let comma = match pair.find(',') {
                None => {
                    return Err(anyhow!("expected comma in {}", pair));
                }
                Some(n) => n,
            };
            let (s1, s2) = pair.split_at(comma);
            let s2 = s2.trim_start_matches(',');
            let n1 = s1.parse::<usize>()?;
            let n2 = s2.parse::<usize>()?;
            println!("pair: {}, {}", n1, n2);
            row.push((n1, n2));

            min_x = if n1 < min_x { n1 } else { min_x };
            min_y = if n2 < min_y { n2 } else { min_y };
            max_x = if n1 > max_x { n1 } else { max_x };
            max_y = if n2 > max_y { n2 } else { max_y };
        }
        data.push(row);
    }
    println!("min = ({}, {})", min_x, min_y);
    println!("max = ({}, {})", max_x, max_y);

    let mut m = Map::new(min_x, min_y, max_x, max_y);

    for row in data {
        let mut previous: Option<(usize, usize)> = None;
        for (n1, n2) in row {
            let current = (n1, n2);
            match previous {
                Some(prev) => {
                    let (start_x, end_x) = if prev.0 < current.0 {
                        (prev.0, current.0)
                    } else {
                        (current.0, prev.0)
                    };
                    let (start_y, end_y) = if prev.1 < current.1 {
                        (prev.1, current.1)
                    } else {
                        (current.1, prev.1)
                    };

                    for y in start_y..=end_y {
                        for x in start_x..=end_x {
                            m.set(x, y, Tile::Rock);
                        }
                    }
                }
                _ => {}
            };
            previous = Some(current);
        }
    }

    Ok(m)
}

fn drop_sand(m: &mut Map, part2: bool) -> bool {
    let mut s = (500, 0);
    loop {
        if s.1 > m.max_y {
            if part2 {
                m.set(s.0, s.1, Tile::Sand);
                return true;
            } else {
                return false;
            }
        }
        if match m.get(s.0, s.1 + 1) {
            Tile::Empty => true,
            _ => false,
        } {
            s.1 = s.1 + 1;
            continue;
        }
        if match m.get(s.0 - 1, s.1 + 1) {
            Tile::Empty => true,
            _ => false,
        } {
            s.0 = s.0 - 1;
            s.1 = s.1 + 1;
            continue;
        }
        if match m.get(s.0 + 1, s.1 + 1) {
            Tile::Empty => true,
            _ => false,
        } {
            s.0 = s.0 + 1;
            s.1 = s.1 + 1;
            continue;
        }
        m.set(s.0, s.1, Tile::Sand);
        if part2 && s.0 == 500 && s.1 == 0 {
            return false;
        }
        return true;
    }
}

fn process(path: &str, part2: bool) -> Result<()> {
    let mut m = read_input(path)?;
    let mut more = true;
    let mut ans = 0;
    while more {
        // println!("");
        // print_map(&m);
        more = drop_sand(&mut m, part2);
        if more {
            ans = ans + 1;
        }
    }
    if part2 {
        ans = ans + 1;
    }
    println!("\nans: {}", ans);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
