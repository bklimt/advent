use advent::common::read_lines;
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
    fn from_char(c: char) -> Self {
        Node {
            n: c == '|' || c == 'J' || c == 'L',
            s: c == '|' || c == '7' || c == 'F',
            e: c == '-' || c == 'L' || c == 'F',
            w: c == '-' || c == 'J' || c == '7',
            d1: None,
            d2: None,
        }
    }

    fn to_char(&self) -> char {
        if let Some(d) = self.d() {
            char::from_digit(d as u32, 36).unwrap_or('#')
        } else if self.n && self.s && !self.e && !self.w {
            '|'
        } else if self.n && self.w && !self.s && !self.e {
            'J'
        } else if self.n && self.e && !self.w && !self.s {
            'L'
        } else if self.w && self.e && !self.n && !self.s {
            '-'
        } else if self.s && self.w && !self.n && !self.e {
            '7'
        } else if self.s && self.e && !self.n && !self.w {
            'F'
        } else if !self.n && !self.s && !self.w && !self.e {
            '.'
        } else {
            panic!("invalid node: {:?}", self);
        }
    }

    fn d(&self) -> Option<i64> {
        if let Some(d1) = self.d1 {
            if let Some(d2) = self.d2 {
                Some(d1.min(d2))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn in_loop(&self) -> bool {
        self.d().is_some()
    }
}

#[derive(Debug)]
struct Input {
    start: (usize, usize),
    map: Vec<Vec<Node>>,
}

impl Input {
    fn read(path: &str, _debug: bool) -> Result<Self> {
        let mut start = (0, 0);
        let mut map = Vec::new();
        for line in read_lines(path)? {
            map.push(line.chars().map(Node::from_char).collect());

            if let Some(s) = line.find('S') {
                start = ((map.len() - 1), s);
            }
        }
        let mut input = Input { start, map };
        input.patch_start();
        Ok(input)
    }

    fn print(&self) {
        for row in self.map.iter() {
            let line: String = row.iter().map(Node::to_char).collect();
            println!("{}", line);
        }
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

    fn patch_start(&mut self) {
        let n = if self.start.0 > 0 {
            let n = self
                .get((self.start.0 - 1, self.start.1))
                .expect("in bounds");
            n.s
        } else {
            false
        };
        let s = if self.start.0 < self.map.len() - 1 {
            let s = self
                .get((self.start.0 + 1, self.start.1))
                .expect("in bounds");
            s.n
        } else {
            false
        };
        let w = if self.start.1 > 0 {
            let w = self
                .get((self.start.0, self.start.1 - 1))
                .expect("in bounds");
            w.e
        } else {
            false
        };
        let e = if self.start.1 < self.map.first().expect("not empty").len() - 1 {
            let e = self
                .get((self.start.0, self.start.1 + 1))
                .expect("in bounds");
            e.w
        } else {
            false
        };

        let node = self.get_mut(self.start).expect("start is valid");
        node.n = n;
        node.s = s;
        node.e = e;
        node.w = w;
        node.d1 = Some(0);
        node.d2 = Some(0);
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

    fn part1(&mut self, debug: bool) -> Result<i64> {
        let (s1, s2) = self.find_starts()?;

        self.propagate_distance(s1, false);
        self.propagate_distance(s2, true);

        if debug {
            self.print();
        }

        let mut ans: Option<i64> = None;
        for row in self.map.iter() {
            for node in row {
                if let Some(d) = node.d() {
                    ans = Some(match ans {
                        Some(d3) => d3.max(d),
                        None => d,
                    });
                }
            }
        }

        Ok(ans.expect("an answer"))
    }

    fn part2(&mut self, _debug: bool) -> usize {
        let mut total: usize = 0;
        for row in self.map.iter() {
            let mut inside = false;
            let mut above_border = false;
            let mut below_border = false;
            for node in row.iter() {
                if node.in_loop() {
                    if node.n && node.s {
                        // |
                        inside = !inside;
                    } else if node.w && node.e {
                        if !above_border && !below_border {
                            panic!("unexpected -");
                        }
                    } else if node.n && node.e {
                        // L
                        if above_border || below_border {
                            panic!("unexpected L");
                        }
                        below_border = true;
                    } else if node.n && node.w {
                        // J
                        if below_border {
                            below_border = false;
                        } else if above_border {
                            // F----J
                            above_border = false;
                            inside = !inside;
                        } else {
                            panic!("saw unexpected J");
                        }
                    } else if node.s && node.e {
                        // F
                        if above_border || below_border {
                            panic!("unexpected F");
                        }
                        above_border = true;
                    } else if node.s && node.w {
                        // 7
                        if above_border {
                            above_border = false;
                        } else if below_border {
                            // L----7
                            below_border = false;
                            inside = !inside;
                        } else {
                            panic!("saw unexpected 7");
                        }
                    }
                } else {
                    if inside {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

fn process(args: &Args) -> Result<()> {
    let mut input = Input::read(&args.input, args.debug)?;
    if args.debug {
        input.print();
        println!("");
    }
    println!("ans1: {}", input.part1(args.debug)?);
    println!("ans2: {}", input.part2(args.debug));
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
