use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(long)]
    debug: bool,
}

fn hash(input: &str) -> i64 {
    let mut total = 0i64;
    let mut hash = 0i64;
    for b in input.bytes() {
        let b = b as i64;
        match b {
            10 | 13 => continue,
            44 => {
                total += hash;
                hash = 0;
            }
            _ => {
                hash += b;
                hash *= 17;
                hash %= 256;
            }
        }
    }
    total += hash;
    total
}

struct LensBox {
    lenses: Vec<(String, i64)>,
}

impl LensBox {
    fn new() -> Self {
        LensBox { lenses: Vec::new() }
    }

    fn upsert(&mut self, label: &str, n: i64) {
        for (s, p) in self.lenses.iter_mut() {
            if *s == label {
                *p = n;
                return;
            }
        }
        self.lenses.push((label.to_owned(), n));
    }

    fn delete(&mut self, label: &str) {
        self.lenses.retain(|(s, _)| s != label);
    }

    fn score(&self, debug: bool) -> i64 {
        let mut total = 0i64;
        for (i, (s, n)) in self.lenses.iter().enumerate() {
            let i = i as i64 + 1;
            if debug {
                println!("{}: {} * {} = {}", s, i, n, i * n);
            }
            total += i * n;
        }
        total
    }

    fn print(&self) {
        for (s, n) in self.lenses.iter() {
            print!("[{} {}] ", s, n);
        }
    }
}

struct Drawer {
    boxes: Vec<LensBox>,
}

impl Drawer {
    fn new() -> Self {
        let mut v = Vec::new();
        v.resize_with(256, || LensBox::new());
        Drawer { boxes: v }
    }

    fn apply(&mut self, cmd: &str, debug: bool) -> Result<()> {
        if debug {
            println!("applying command {}", cmd);
        }
        if let Some(i) = cmd.find('-') {
            let cmd = &cmd[..i];
            let b = hash(cmd) as usize;
            self.boxes
                .get_mut(b)
                .expect("hash must be in [0, 256)")
                .delete(cmd);
            Ok(())
        } else if let Some(eq) = cmd.find('=') {
            let n = cmd[eq + 1..].parse::<i64>()?;
            let cmd = &cmd[..eq];
            let b = hash(cmd) as usize;
            self.boxes
                .get_mut(b)
                .expect("hash must be in [0, 256)")
                .upsert(cmd, n);
            Ok(())
        } else {
            Err(anyhow!("invalid command: {}", cmd))
        }
    }

    fn score(&self, debug: bool) -> i64 {
        let mut total = 0i64;
        for (i, b) in self.boxes.iter().enumerate() {
            if b.lenses.len() == 0 {
                continue;
            }
            let i = i as i64 + 1;
            let s = b.score(debug);
            if debug {
                println!("{} * {} = {}", i, s, i * s);
            }
            total += i * s;
        }
        total
    }

    fn print(&self) {
        for (i, b) in self.boxes.iter().enumerate() {
            if b.lenses.len() == 0 {
                continue;
            }
            print!("Box {}: ", i);
            b.print();
            println!("");
        }
    }
}

fn part2(input: &str, debug: bool) -> Result<i64> {
    let input = input.trim();
    let mut drawer = Drawer::new();
    for op in input.split(',') {
        drawer.apply(op, debug)?;
        if debug {
            drawer.print();
            println!("");
        }
    }
    Ok(drawer.score(debug))
}

fn process_file(path: &str, debug: bool) -> Result<()> {
    let mut file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let ans1 = hash(&s[..]);
    println!("ans1: {}", ans1);

    let ans2 = part2(&s[..], debug)?;
    println!("ans2: {}", ans2);

    Ok(())
}

fn process(args: &Args) -> Result<()> {
    process_file(&args.input, args.debug)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
