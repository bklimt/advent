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

    #[arg(long)]
    debug: bool,
}

struct Tube {
    sum: i64,
    cycle: i64,
    register: i64,
    debug: bool,
}

impl Tube {
    fn new(debug: bool) -> Self {
        Tube {
            sum: 0,
            cycle: 0,
            register: 1,
            debug: debug,
        }
    }

    fn visit(&mut self) {
        if self.debug {
            println!("cycle: {:5}, register: {:10}", self.cycle, self.register);
        }
        // 20th, 60th, 100th, 140th, 180th, and 220th cycles
        if self.cycle == 20
            || self.cycle == 60
            || self.cycle == 100
            || self.cycle == 140
            || self.cycle == 180
            || self.cycle == 220
        {
            self.sum = self.sum + (self.cycle * self.register);
        }
    }

    fn cycle(&mut self) {
        self.cycle = self.cycle + 1;
        self.visit();
    }

    fn sum(&self) -> i64 {
        self.sum
    }
}

fn process(path: &str, debug: bool) -> Result<()> {
    let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut r = BufReader::new(file);

    let mut tube = Tube::new(debug);

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
            println!("{}", line);
        }

        if line == "noop" {
            tube.cycle();
        } else if line.starts_with("addx ") {
            let (_, nstr) = line.split_at(5);
            let n = nstr
                .parse::<i64>()
                .with_context(|| format!("invalid number: {}", nstr))?;
            tube.cycle();
            tube.cycle();
            tube.register = tube.register + n;
        } else {
            return Err(anyhow!("invalid line: {}", line));
        }
    }
    tube.cycle();

    println!("sum = {}", tube.sum());

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args.path, args.debug) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
