use anyhow::{anyhow, Context, Result};
use clap::Parser;
use priority_queue::DoublePriorityQueue;
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

struct Map {
    start: (usize, usize),
    end: (usize, usize),
    elevation: Vec<Vec<u32>>,
}

fn read_input(path: &str) -> Result<Map> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut map = Map {
        start: (0, 0),
        end: (0, 0),
        elevation: Vec::new(),
    };
    let mut width: Option<usize> = None;
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

        let mut row: Vec<u32> = Vec::new();
        for (column, c) in line.chars().enumerate() {
            if c == 'S' {
                map.start = (map.elevation.len(), column);
            }
            if c == 'E' {
                map.end = (map.elevation.len(), column);
            }
            let n = match c {
                'S' => 0,
                'E' => 25,
                _ => (c as u32) - ('a' as u32),
            };
            row.push(n);
        }
        if width.is_some() {
            if row.len() != width.unwrap() {
                return Err(anyhow!(
                    "invalid row length: {} vs {}",
                    row.len(),
                    width.unwrap()
                ));
            }
        } else {
            width = Some(row.len());
        }
        map.elevation.push(row);
    }
    if map.elevation.len() == 0 {
        return Err(anyhow!("data is empty"));
    }
    Ok(map)
}

fn can_move(m: &Map, p1: (usize, usize), p2: (usize, usize)) -> bool {
    ((m.elevation[p2.0][p2.1] as i32) - (m.elevation[p1.0][p1.1] as i32)) <= 1
}

fn neighbors(m: &Map, p: (usize, usize)) -> Vec<(usize, usize)> {
    let h = m.elevation.len();
    let w = m.elevation[0].len();
    let mut n = Vec::new();
    if p.0 > 0 && can_move(&m, p, (p.0 - 1, p.1)) {
        n.push((p.0 - 1, p.1))
    }
    if p.1 > 0 && can_move(&m, p, (p.0, p.1 - 1)) {
        n.push((p.0, p.1 - 1))
    }
    if p.0 < (h - 1) && can_move(&m, p, (p.0 + 1, p.1)) {
        n.push((p.0 + 1, p.1))
    }
    if p.1 < (w - 1) && can_move(&m, p, (p.0, p.1 + 1)) {
        n.push((p.0, p.1 + 1))
    }
    n
}

fn dijkstra(m: &Map) -> i32 {
    let h = m.elevation.len();
    let w = m.elevation[0].len();
    let mut dist: Vec<Vec<i32>> = Vec::new();
    for _ in 0..h {
        let mut r: Vec<i32> = Vec::new();
        for _ in 0..w {
            r.push(i32::MAX);
        }
        dist.push(r);
    }

    dist[m.start.0][m.start.1] = 0;

    let mut q = DoublePriorityQueue::new();
    for r in 0..h {
        for c in 0..w {
            q.push((r, c), dist[r][c]);
        }
    }
    while !q.is_empty() {
        let (u, d) = q.pop_min().unwrap();
        println!("visiting {:3}, {:3} = {}", u.0, u.1, d);
        let n = neighbors(&m, u);
        if d != i32::MAX {
            let alt = d + 1;
            for v in n {
                println!("neighbor {:3}, {:3} = {}", v.0, v.1, dist[v.0][v.1]);
                if alt < dist[v.0][v.1] {
                    println!("updating to {}", alt);
                    dist[v.0][v.1] = alt;
                    q.push_decrease(v, alt);
                }
            }
        }
    }
    dist[m.end.0][m.end.1]
}

fn print_elevation(data: &Vec<Vec<u32>>) {
    for row in data.iter() {
        for col in row.iter() {
            print!("{:3}", *col);
        }
        println!("");
    }
}

fn process(path: &str, _part2: bool) -> Result<()> {
    let map = read_input(path)?;
    println!("start: {:3}, {:3}", map.start.0, map.start.1);
    println!("  end: {:3}, {:3}", map.end.0, map.end.1);
    // print_elevation(&map.elevation);
    println!("  ans: {:3}", dijkstra(&map));
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
