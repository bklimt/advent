use anyhow::{anyhow, Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::VecDeque;
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

#[derive(Debug)]
struct Node {
    n: bool,
    s: bool,
    e: bool,
    w: bool,
    d1: Option<i64>,
    d2: Option<i64>,
}

impl Node {
    fn from_str(c: char) -> Self {
        Node {
            n: c == '|' || c == 'J' || c == 'L',
            s: c == '|' || c == '7' || c == 'F',
            e: c == '-' || c == 'L' || c == 'F',
            w: c == '-' || c == 'J' || c == '7',
            d1: None,
            d2: None,
        }
    }
}

#[derive(Debug)]
struct Input {
    start: (usize, usize),
    map: Vec<Vec<Node>>,
}

impl Input {
    fn read(path: &str, _debug: bool) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
        let mut r = BufReader::new(file);
        let mut start = (0, 0);
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

            map.push(line.chars().map(Node::from_str).collect());

            if let Some(s) = line.find('S') {
                start = ((map.len() - 1), s);
            }
        }
        Ok(Input { start, map })
    }

    // row-major order
    fn get(&self, row_col: (usize, usize)) -> Option<&Node> {
        match self.map.get(row_col.0) {
            Some(v) => v.get(row_col.1),
            None => None,
        }
    }

    // row-major order
    fn get_mut(&mut self, row_col: (usize, usize)) -> Option<&mut Node> {
        match self.map.get_mut(row_col.0) {
            Some(v) => v.get_mut(row_col.1),
            None => None,
        }
    }

    fn find_starts(&self) -> Result<((usize, usize), (usize, usize))> {
        let mut possible = Vec::new();
        if self.start.0 > 0 {
            let n = self
                .get((self.start.0 - 1, self.start.1))
                .expect("in bounds");
            if n.s {
                possible.push((self.start.0 - 1, self.start.1));
            }
        }
        if self.start.0 < self.map.len() - 1 {
            let s = self
                .get((self.start.0 + 1, self.start.1))
                .expect("in bounds");
            if s.n {
                possible.push((self.start.0 + 1, self.start.1));
            }
        }
        if self.start.1 > 0 {
            let w = self
                .get((self.start.0, self.start.1 - 1))
                .expect("in bounds");
            if w.e {
                possible.push((self.start.0, self.start.1 - 1));
            }
        }
        if self.start.1 < self.map.first().expect("not empty").len() - 1 {
            let e = self
                .get((self.start.0, self.start.1 + 1))
                .expect("in bounds");
            if e.w {
                possible.push((self.start.0, self.start.1 + 1));
            }
        }
        if possible.len() != 2 {
            return Err(anyhow!("invalid possible start positions: {:?}", possible));
        }
        Ok(possible.into_iter().collect_tuple().expect("len == 2"))
    }

    fn propagate_distance(&mut self, start: (usize, usize), is_d2: bool) {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back(start);
        let n = self.get_mut(start).expect("start must be valid");
        if is_d2 {
            n.d2 = Some(1);
        } else {
            n.d1 = Some(1);
        }

        while let Some(p) = q.pop_front() {
            // Consider the node at p. Can it improve any of its neighbors?
            if let Some(n) = self.get_mut(p) {
                let d1 = if is_d2 { n.d2 } else { n.d1 }.expect("should be reachable");
                let mut possible = Vec::new();
                if n.n {
                    possible.push((p.0 - 1, p.1));
                }
                if n.s {
                    possible.push((p.0 + 1, p.1));
                }
                if n.e {
                    possible.push((p.0, p.1 + 1));
                }
                if n.w {
                    possible.push((p.0, p.1 - 1));
                }
                for p2 in possible {
                    if let Some(n2) = self.get_mut(p2) {
                        if is_d2 {
                            match n2.d2 {
                                Some(d2) => {
                                    if d1 + 1 < d2 {
                                        n2.d2 = Some(d1 + 1);
                                        q.push_back(p2);
                                    }
                                }
                                None => {
                                    n2.d2 = Some(d1 + 1);
                                    q.push_back(p2);
                                }
                            }
                        } else {
                            match n2.d1 {
                                Some(d2) => {
                                    if d1 + 1 < d2 {
                                        n2.d1 = Some(d1 + 1);
                                        q.push_back(p2);
                                    }
                                }
                                None => {
                                    n2.d1 = Some(d1 + 1);
                                    q.push_back(p2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Result<i64> {
        let (s1, s2) = self.find_starts()?;

        self.propagate_distance(s1, false);
        self.propagate_distance(s2, true);

        let mut ans: Option<i64> = None;
        for row in self.map.iter() {
            for node in row {
                if let Some(d1) = node.d1 {
                    if let Some(d2) = node.d2 {
                        let d = d1.min(d2);
                        if let Some(d3) = ans {
                            ans = Some(d3.max(d));
                        } else {
                            ans = Some(d);
                        }
                    }
                }
            }
        }

        Ok(ans.expect("an answer"))
    }
}

fn process(args: &Args) -> Result<()> {
    let mut input = Input::read(&args.input, args.debug)?;
    println!("ans1: {}", input.part1()?);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
