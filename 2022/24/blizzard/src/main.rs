use anyhow::{anyhow, Context, Result};
use clap::Parser;
use priority_queue::DoublePriorityQueue;
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
}

fn modulo(n: i64, d: i64) -> i64 {
    ((n % d) + d) % d
}

enum Tile {
    Empty,
    Wall,
    Up,
    Down,
    Left,
    Right,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '^' => Tile::Up,
            'v' => Tile::Down,
            '<' => Tile::Left,
            '>' => Tile::Right,
            _ => {
                return Err(anyhow!("invalid character: {}", c));
            }
        })
    }
}

fn read_input(path: &str, debug: bool) -> Result<Vec<Vec<Tile>>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut map = Vec::new();
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

        if debug {
            println!("line: {}", line);
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c)?);
        }
        map.push(row);
    }
    Ok(map)
}

struct Map {
    width: usize,
    height: usize,
    up_blizzards: HashMap<usize, Vec<usize>>,
    down_blizzards: HashMap<usize, Vec<usize>>,
    left_blizzards: HashMap<usize, Vec<usize>>,
    right_blizzards: HashMap<usize, Vec<usize>>,
}

fn build_map(tiles: &Vec<Vec<Tile>>) -> Result<Map> {
    // Check the dimensions.
    if tiles.len() < 3 {
        return Err(anyhow!("no rows"));
    }
    let height = tiles.len();
    let width = tiles[0].len();
    for row in tiles.iter() {
        if row.len() != width {
            return Err(anyhow!("invalid line width: {}", row.len()));
        }
    }

    let mut up_blizzards: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut down_blizzards: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut left_blizzards: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut right_blizzards: HashMap<usize, Vec<usize>> = HashMap::new();

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Empty => {}
                Tile::Wall => {}
                Tile::Up => {
                    up_blizzards.entry(x).or_default().push(y);
                }
                Tile::Down => {
                    down_blizzards.entry(x).or_default().push(y);
                }
                Tile::Left => {
                    left_blizzards.entry(y).or_default().push(x);
                }
                Tile::Right => {
                    right_blizzards.entry(y).or_default().push(x);
                }
            }
        }
    }

    Ok(Map {
        width,
        height,
        up_blizzards,
        down_blizzards,
        left_blizzards,
        right_blizzards,
    })
}

impl Map {
    fn has_blizzard(&self, x: usize, y: usize, t: usize) -> bool {
        // TODO(klimt): This modulo is wrong...
        if let Some(ys) = self.up_blizzards.get(&x) {
            for y_0 in ys {
                let y_t = (modulo(((*y_0 as i64) - (t as i64)) - 1, (self.height - 2) as i64) + 1)
                    as usize;
                if y_t == y {
                    return true;
                }
            }
        }
        if let Some(ys) = self.down_blizzards.get(&x) {
            for y_0 in ys {
                let y_t = (modulo(((*y_0 as i64) + (t as i64)) - 1, (self.height as i64) - 2) + 1)
                    as usize;
                if y_t == y {
                    return true;
                }
            }
        }
        if let Some(xs) = self.left_blizzards.get(&y) {
            for x_0 in xs {
                let x_t = (modulo(((*x_0 as i64) - (t as i64)) - 1, (self.width - 2) as i64) + 1)
                    as usize;
                if x_t == x {
                    return true;
                }
            }
        }
        if let Some(xs) = self.right_blizzards.get(&y) {
            for x_0 in xs {
                let x_t = (modulo(((*x_0 as i64) + (t as i64)) - 1, (self.width - 2) as i64) + 1)
                    as usize;
                if x_t == x {
                    return true;
                }
            }
        }
        false
    }

    fn is_traversable(&self, x: usize, y: usize, t: usize) -> bool {
        return if x == 1 && y == 0 {
            true
        } else if x == self.width - 2 && y == self.height - 1 {
            true
        } else if x == 0 {
            false
        } else if y == 0 {
            false
        } else if x >= self.width - 1 {
            false
        } else if y >= self.height - 1 {
            false
        } else {
            !self.has_blizzard(x, y, t)
        };
    }

    fn adjacent(&self, node: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
        let (x, y, t) = node;
        let mut adj = Vec::new();
        // Wait
        if self.is_traversable(x, y, t + 1) {
            adj.push((x, y, t + 1))
        }
        // Up
        if y > 0 && self.is_traversable(x, y - 1, t + 1) {
            adj.push((x, y - 1, t + 1));
        }
        // Down
        if self.is_traversable(x, y + 1, t + 1) {
            adj.push((x, y + 1, t + 1));
        }
        // Left
        if x > 0 && self.is_traversable(x - 1, y, t + 1) {
            adj.push((x - 1, y, t + 1));
        }
        // Right
        if self.is_traversable(x + 1, y, t + 1) {
            adj.push((x + 1, y, t + 1));
        }
        adj
    }

    fn dijkstra(&self, debug: bool) {
        let mut max_t = 0usize;
        let mut q = DoublePriorityQueue::new();
        let mut dist = HashMap::new();
        q.push((1, 0, 0), 0i64);
        dist.insert((1, 0, 0), 0i64);
        while let Some((u, d_u)) = q.pop_min() {
            if debug {
                if u.2 > max_t {
                    max_t = u.2;
                    println!("t = {}", max_t);
                }
            }
            if u.0 == self.width - 2 && u.1 == self.height - 1 {
                if debug {
                    println!("t = {}", u.2);
                }
                break;
            }
            for v in self.adjacent(u) {
                let alt = d_u + 1;
                if alt < *dist.get(&v).unwrap_or(&i64::MAX) {
                    dist.insert(v.clone(), alt);
                    q.push_decrease(v.clone(), alt);
                }
            }
        }
    }
}

fn process(args: &Args) -> Result<()> {
    let debug = args.debug;
    if debug {
        println!("reading tiles...");
    }
    let tiles = read_input(&args.input, args.debug)?;

    if debug {
        println!("building map...");
    }
    let map = build_map(&tiles)?;

    if debug {
        println!("searching...");
    }
    map.dijkstra(debug);

    if debug {
        println!("done.");
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
