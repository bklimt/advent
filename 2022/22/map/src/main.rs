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
}

fn read_input(path: &str, debug: bool) -> Result<()> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);
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
    }
    Ok(())
}

fn process(args: &Args) -> Result<()> {
    println!("reading input...");
    let _ = read_input(&args.input, args.debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
