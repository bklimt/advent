use anyhow::{Context, Result};
use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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
struct Record {
    text: String,
    counts: Vec<usize>,
}

impl Record {
    fn expand(&self) -> Self {
        let t = self.text.as_str();
        let text = vec![t, t, t, t, t].join("?");
        let mut counts = Vec::new();
        for _ in 0..5 {
            for &n in self.counts.iter() {
                counts.push(n);
            }
        }
        Record { text, counts }
    }
}

fn parse_num_list(line: &str) -> Result<Vec<usize>> {
    let mut v = Vec::new();
    for part in line.split(',') {
        let part = part.trim();
        v.push(
            part.parse()
                .context(format!("invalid number in {:?}", line))?,
        );
    }
    Ok(v)
}

fn read_input(path: &str, _debug: bool) -> Result<Vec<Record>> {
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

        let space = line.find(' ').context(format!("invalid line: {}", line))?;
        let (text, count_text) = line.split_at(space);
        let text = text.to_owned();
        let count_text = &count_text[1..];
        let counts = parse_num_list(count_text)?;
        v.push(Record { text, counts });
    }

    Ok(v)
}

// Returns true if tail matches (.|?)*.
fn parse_tail(line: &[char], debug: bool) -> bool {
    if debug {
        println!("parse_tail({:?})", line.iter().collect::<String>());
    }

    for &c in line {
        if c != '.' && c != '?' {
            if debug {
                println!("<- false (invalid tail)");
            }
            return false;
        }
    }
    if debug {
        println!("<- true");
    }
    true
}

// Returns the number of ways the line can match starting with the first number.
fn parse_pounds(line: &[char], nums: &[usize], debug: bool) -> usize {
    if debug {
        println!(
            "parse_pounds({:?}, {:?})",
            line.iter().collect::<String>(),
            nums
        );
    }

    // This should never get called when no numbers are expected.
    if nums.len() == 0 {
        panic!("got a line with no numbers");
    }

    // As an optimization, fail now if the rest isn't long enough.
    let mut min_len = 0usize;
    for &n in nums.iter() {
        min_len += n as usize;
    }
    min_len += nums.len() - 1;
    if line.len() < min_len {
        if debug {
            println!("<- 0 (line.len() = {} < min_len = {})", line.len(), min_len);
        }
        return 0;
    }

    // Check the first number.
    let &n = nums.first().expect("len > 0");
    for i in 0..n {
        if line[i] != '#' && line[i] != '?' {
            if debug {
                println!("<- 0 ({} is not a pound)", line[i]);
            }
            return 0;
        }
    }
    // If that's the only number, then check we only have dots after that.
    if nums.len() == 1 {
        let ans = if parse_tail(&line[n..], debug) { 1 } else { 0 };
        if debug {
            println!("<- {} (parse_pounds)", ans);
        }
        return ans;
    }

    // Check that the number is followed by a dot.
    if line[n] != '.' && line[n] != '?' {
        if debug {
            println!("<- 0 (no trailing dot)");
        }
        return 0;
    }

    // Parse the rest.
    let n = n + 1;
    let ans = parse_dots(&line[n..], &nums[1..], debug);
    if debug {
        println!("<- {} (parse_pounds)", ans);
    }
    ans
}

fn parse_dots(line: &[char], nums: &[usize], debug: bool) -> usize {
    if debug {
        println!(
            "parse_dots({:?}, {:?})",
            line.iter().collect::<String>(),
            nums
        );
    }
    if nums.len() == 0 {
        panic!("got a line with no numbers");
    }
    if line.len() == 0 {
        if debug {
            println!("<- 0 (empty line)");
        }
        return 0;
    }
    let ans = match line[0] {
        '.' => parse_dots(&line[1..], nums, debug),
        '#' => parse_pounds(&line[..], nums, debug),
        '?' => parse_dots(&line[1..], nums, debug) + parse_pounds(&line[..], nums, debug),
        _ => panic!("invalid char: {}", line[0]),
    };
    if debug {
        println!("<- {} (parse_dots)", ans);
    }
    ans
}

fn process_record(record: &Record, debug: bool) -> usize {
    let line = record.text.chars().collect_vec();
    let nums = record.counts.clone();
    let n = parse_dots(&line[..], &nums[..], false);
    if debug {
        println!("{:?} -> {}", record, n);
        // println!("\n");
    }
    n
}

fn process(args: &Args) -> Result<()> {
    let input = read_input(&args.input, args.debug)?;
    let mut total1 = 0usize;
    let mut total2 = 0usize;
    for record in input.iter() {
        total1 += process_record(record, args.debug);

        let expanded = record.expand();
        total2 += process_record(&expanded, args.debug);

        // print!(".");
        // io::stdout().flush().unwrap();
    }
    println!("");
    println!("ans 1 = {}", total1);
    println!("ans 2 = {}", total2);
    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
