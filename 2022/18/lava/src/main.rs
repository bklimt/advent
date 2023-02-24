use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize, usize);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn parse_line(s: &str) -> Result<Coord> {
    let comma = s
        .find(',')
        .ok_or_else(|| anyhow!("missing first comma: {}", s))?;
    let (sn1, s) = s.split_at(comma);
    let s = s
        .strip_prefix(",")
        .ok_or_else(|| anyhow!("unable to strip first comma: {}", s))?;
    let comma = s
        .find(',')
        .ok_or_else(|| anyhow!("missing second comma: {}", s))?;
    let (sn2, s) = s.split_at(comma);
    let sn3 = s
        .strip_prefix(",")
        .ok_or_else(|| anyhow!("unable to strip second comma: {}", s))?;

    let n1 = sn1
        .parse::<usize>()
        .with_context(|| anyhow!("invalid number: {}", sn1))?;
    let n2 = sn2
        .parse::<usize>()
        .with_context(|| anyhow!("invalid number: {}", sn2))?;
    let n3 = sn3
        .parse::<usize>()
        .with_context(|| anyhow!("invalid number: {}", sn3))?;

    Ok((n1, n2, n3))
}

fn read_input(path: &str, debug: bool) -> Result<Vec<Coord>> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    let mut v = Vec::new();
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

        v.push(parse_line(line)?);
    }
    Ok(v)
}

fn dim(v: &Vec<Coord>) -> Coord {
    let (mut x, mut y, mut z) = v.first().unwrap();
    for c in v.iter() {
        x = x.max(c.0 + 2);
        y = y.max(c.1 + 2);
        z = z.max(c.2 + 2);
    }
    (x, y, z)
}

struct Space {
    filled: bool,
    exposed: bool,
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let coords = read_input(&args.input, args.debug)?;

    let dim = dim(&coords);
    println!("dim = {:?}", dim);

    println!("building space...");
    let mut cube = Vec::new();
    for _ in 0..dim.0 {
        let mut square = Vec::new();
        for _ in 0..dim.1 {
            let mut line = Vec::new();
            for _ in 0..dim.2 {
                line.push(Space {
                    filled: false,
                    exposed: false,
                });
            }
            square.push(line);
        }
        cube.push(square);
    }

    println!("adding coords...");
    for c in coords.iter() {
        cube[c.0 as usize][c.1 as usize][c.2 as usize].filled = true;
    }

    println!("counting faces...");
    let mut total = 0;
    for c in coords.iter() {
        let mut faces = 0;
        if c.0 == 0 || !cube[c.0 - 1][c.1][c.2].filled {
            faces = faces + 1;
        }
        if !cube[c.0 + 1][c.1][c.2].filled {
            faces = faces + 1;
        }
        if c.1 == 0 || !cube[c.0][c.1 - 1][c.2].filled {
            faces = faces + 1;
        }
        if !cube[c.0][c.1 + 1][c.2].filled {
            faces = faces + 1;
        }
        if c.2 == 0 || !cube[c.0][c.1][c.2 - 1].filled {
            faces = faces + 1;
        }
        if !cube[c.0][c.1][c.2 + 1].filled {
            faces = faces + 1;
        }
        total = total + faces;
    }
    println!("part 1 ans = {}", total);

    let mut q = VecDeque::new();

    println!("seeding exposed at x bounds...");
    for y in 0..dim.1 {
        for z in 0..dim.2 {
            q.push_back((0, y, z));
            q.push_back((dim.0 - 1, y, z));
        }
    }
    println!("seeding exposed at y bounds...");
    for x in 0..dim.0 {
        for z in 0..dim.2 {
            q.push_back((x, 0, z));
            q.push_back((x, dim.1 - 1, z));
        }
    }
    println!("seeding exposed at z bounds...");
    for x in 0..dim.0 {
        for y in 0..dim.1 {
            q.push_back((x, y, 0));
            q.push_back((x, y, dim.2 - 1));
        }
    }

    println!("doing flood fill...");
    while let Some(c) = q.pop_front() {
        if cube[c.0][c.1][c.2].exposed {
            continue;
        }
        cube[c.0][c.1][c.2].exposed = true;

        if cube[c.0][c.1][c.2].filled {
            continue;
        }

        if c.0 > 0 && !cube[c.0 - 1][c.1][c.2].exposed {
            q.push_back((c.0 - 1, c.1, c.2));
        }
        if c.1 > 0 && !cube[c.0][c.1 - 1][c.2].exposed {
            q.push_back((c.0, c.1 - 1, c.2));
        }
        if c.2 > 0 && !cube[c.0][c.1][c.2 - 1].exposed {
            q.push_back((c.0, c.1, c.2 - 1));
        }

        if c.0 < dim.0 - 1 && !cube[c.0 + 1][c.1][c.2].exposed {
            q.push_back((c.0 + 1, c.1, c.2));
        }
        if c.1 < dim.1 - 1 && !cube[c.0][c.1 + 1][c.2].exposed {
            q.push_back((c.0, c.1 + 1, c.2));
        }
        if c.2 < dim.2 - 1 && !cube[c.0][c.1][c.2 + 1].exposed {
            q.push_back((c.0, c.1, c.2 + 1));
        }
    }

    println!("counting exposed faces...");
    total = 0;
    for c in coords.iter() {
        let mut faces = 0;
        if c.0 == 0 || !cube[c.0 - 1][c.1][c.2].filled && cube[c.0 - 1][c.1][c.2].exposed {
            faces = faces + 1;
        }
        if !cube[c.0 + 1][c.1][c.2].filled && cube[c.0 + 1][c.1][c.2].exposed {
            faces = faces + 1;
        }
        if c.1 == 0 || !cube[c.0][c.1 - 1][c.2].filled && cube[c.0][c.1 - 1][c.2].exposed {
            faces = faces + 1;
        }
        if !cube[c.0][c.1 + 1][c.2].filled && cube[c.0][c.1 + 1][c.2].exposed {
            faces = faces + 1;
        }
        if c.2 == 0 || !cube[c.0][c.1][c.2 - 1].filled && cube[c.0][c.1][c.2 - 1].exposed {
            faces = faces + 1;
        }
        if !cube[c.0][c.1][c.2 + 1].filled && cube[c.0][c.1][c.2 + 1].exposed {
            faces = faces + 1;
        }
        total = total + faces;
    }
    println!("part 2 ans = {}", total);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
