use anyhow::{anyhow, Context, Result};
use clap::Parser;
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

pub fn print_stacks(stacks: &Vec<String>) -> Result<()> {
    let max_depth = stacks
        .iter()
        .map(|s| s.len())
        .max()
        .ok_or(anyhow!("no stacks"))?;

    for i in 1..=max_depth {
        for stack in stacks.iter() {
            if max_depth - i < stack.len() {
                print!("[{}] ", stack.as_bytes()[max_depth - i] as char);
            } else {
                print!("    ");
            }
        }
        println!("");
    }
    for i in 0..stacks.len() {
        if i == 0 {
            print!("    ");
        } else {
            print!(" {}  ", i);
        }
    }
    println!("\n");

    Ok(())
}

pub fn process_instructions(stacks: &mut Vec<String>, path: &str) -> Result<()> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
    loop {
        print_stacks(&stacks)?;

        let mut line = String::new();
        let n = r.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "" {
            if n == 0 {
                break;
            }
            continue;
        }

        println!("{}", line);

        let line = line
            .strip_prefix("move ")
            .ok_or_else(|| anyhow!("missing \"move \": {:?}", line))?;

        let space = line
            .find(' ')
            .ok_or_else(|| anyhow!("no space: {:?}", line))?;

        let (amount_str, line) = line.split_at(space);
        let amount = amount_str
            .parse::<u32>()
            .with_context(|| format!("invalid amount: {:?}", amount_str))?;

        let line = line
            .strip_prefix(" from ")
            .ok_or_else(|| anyhow!("missing \" from \": {:?}", line))?;

        let space = line
            .find(' ')
            .ok_or_else(|| anyhow!("no space: {:?}", line))?;

        let (src_str, line) = line.split_at(space);
        let src = src_str
            .parse::<usize>()
            .with_context(|| format!("invalid source: {:?}", line))?;

        let dst_str = line
            .strip_prefix(" to ")
            .ok_or_else(|| anyhow!("missing \" to \": {:?}", line))?;

        let dst = dst_str
            .parse::<usize>()
            .with_context(|| format!("invalid dst: {:?}", dst_str))?;

        if src >= stacks.len() || dst >= stacks.len() {
            return Err(anyhow!("invalid move from {} to {}", src, dst));
        }

        for _ in 0..amount {
            let stack = stacks
                .get_mut(src)
                .ok_or_else(|| anyhow!("missing stack: {}", src))?;

            let item = stack.pop().ok_or_else(|| anyhow!("empty stack"))?;

            stacks
                .get_mut(dst)
                .ok_or_else(|| anyhow!("missing stack: {}", dst))?
                .push(item);
        }
    }

    Ok(())
}

pub fn process(path: &str, _part2: bool) -> Result<()> {
    /*
                  [J]             [B] [W]
                  [T]     [W] [F] [R] [Z]
              [Q] [M]     [J] [R] [W] [H]
          [F] [L] [P]     [R] [N] [Z] [G]
      [F] [M] [S] [Q]     [M] [P] [S] [C]
      [L] [V] [R] [V] [W] [P] [C] [P] [J]
      [M] [Z] [V] [S] [S] [V] [Q] [H] [M]
      [W] [B] [H] [F] [L] [F] [J] [V] [B]
       1   2   3   4   5   6   7   8   9
    */
    let mut stacks: Vec<String> = Vec::new();
    stacks.push("".to_string());
    stacks.push("WMLF".to_string());
    stacks.push("BZVMF".to_string());
    stacks.push("HVRSLQ".to_string());
    stacks.push("FSVQPMTJ".to_string());
    stacks.push("LSW".to_string());
    stacks.push("FVPMRJW".to_string());
    stacks.push("JQCPNRF".to_string());
    stacks.push("VHPSZWRB".to_string());
    stacks.push("BMJCGHZW".to_string());

    process_instructions(&mut stacks, path)?;

    for stack in stacks.iter() {
        if stack.len() == 0 {
            continue;
        }
        print!("{:}", stack.as_bytes()[stack.len() - 1] as char);
    }
    println!("");

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
