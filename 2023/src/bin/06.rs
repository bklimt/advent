use anyhow::Result;
use clap::Parser;
use std::iter::zip;
use std::option::Option;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    debug: bool,
}

fn compute_charge(time: i64, distance: i64, debug: bool) -> i64 {
    // distance = (time - charge) * charge
    // 0 = -charge^2 + charge*time + -distance
    let t = time as f64;
    let d = distance as f64 + 0.00001;
    let charge1 = (-t + (t * t - 4.0 * d).sqrt()) / -2.0;
    let charge2 = (-t - (t * t - 4.0 * d).sqrt()) / -2.0;
    let minimum = charge1.ceil() as i64;
    let maximum = charge2.floor() as i64;
    let score = (maximum - minimum) + 1;
    if debug {
        println!(
            "time: {}, distance: {}, charge1: {}, charge2: {}, min: {}, max: {}, score: {}",
            time, distance, charge1, charge2, minimum, maximum, score
        );
    }
    score
}

fn part1(times: Vec<i64>, distances: Vec<i64>, debug: bool) -> i64 {
    let mut score = 1;
    let zipped = zip(times, distances);
    for (time, distance) in zipped {
        score *= compute_charge(time, distance, debug);
    }
    score
}

fn process(args: &Args) -> Result<()> {
    let times1 = vec![7, 15, 30];
    let distances1 = vec![9, 40, 200];
    println!("sample 1: {}", part1(times1, distances1, args.debug));

    let times2 = vec![34, 90, 89, 86];
    let distances2 = vec![204, 1713, 1210, 1780];
    println!("input 1: {}", part1(times2, distances2, args.debug));

    println!("sample 2: {}", compute_charge(71530, 940200, args.debug));
    println!(
        "input 2: {}",
        compute_charge(34908986, 204171312101780, args.debug)
    );

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(&args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
