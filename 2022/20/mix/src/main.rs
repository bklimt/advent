use anyhow::{Context, Result};
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

struct Node {
    id: u64,
    val: i64,
}

fn read_input(path: &str, debug: bool) -> Result<Vec<i64>> {
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

        v.push(line.parse::<i64>()?);
    }
    Ok(v)
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let input = read_input(&args.input, args.debug)?;

    // Assign ids to each number.
    let mut nodes = Vec::new();
    let mut rearranged = Vec::new();
    for original_val in input {
        let id = rand::random();
        let val = if args.part2 {
            811589153 * original_val
        } else {
            original_val
        };
        nodes.push(Node { id, val });
        rearranged.push(id);
        if args.debug {
            println!("{{ id: {}, val: {} }} ", id, val);
        }
    }
    if args.debug {
        println!("");
    }

    // Rearrange them as specified.
    let iterations = if args.part2 { 10 } else { 1 };
    for _ in 0..iterations {
        for n in nodes.iter() {
            let i = rearranged.iter().position(|id| *id == n.id).unwrap();
            // It's -1 because of a bug in the problem spec.
            let len = (nodes.len() - 1) as i64;
            let mut j = (((((i as i64) + n.val) % len) + len) % len) as usize;
            if args.debug {
                println!("moving {} from {} to {} with id={}", n.val, i, j, n.id);
            }

            // No element is allowed to move to the front of the list.
            if j == 0 && i != 0 {
                j = nodes.len() - 1;
            }

            rearranged.remove(i);
            rearranged.insert(j, n.id);

            if args.debug {
                print!("new rearranged: ");
                for id in rearranged.iter() {
                    print!("{} ", id);
                }
                println!("");
                for id in rearranged.iter() {
                    let i = nodes.iter().position(|n| n.id == *id).unwrap();
                    print!("{} ", nodes[i].val);
                }
                println!("");
            }
        }
    }

    // Reconstruct the final array.
    let mut result = Vec::new();
    for id in rearranged {
        let i = nodes.iter().position(|n| n.id == id).unwrap();
        result.push(nodes[i].val);
        if args.debug {
            print!("{} ", nodes[i].val);
        }
    }
    println!("");

    let zero = result.iter().position(|n| *n == 0).unwrap();
    let len = result.len();
    let ans =
        result[(zero + 1000) % len] + result[(zero + 2000) % len] + result[(zero + 3000) % len];
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
