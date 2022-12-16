use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    part2: bool,
}

struct Monkey {
    init: Vec<i32>,
    op: fn(i32) -> i32,
    test: i32,
    yes: i32,
    no: i32,
}

fn init_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    monkeys.push(Monkey {
        init: vec![62, 92, 50, 63, 62, 93, 73, 50],
        op: |old| old * 7,
        test: 2,
        yes: 7,
        no: 1,
    });

    monkeys.push(Monkey {
        init: vec![51, 97, 74, 84, 99],
        op: |old| old + 3,
        test: 7,
        yes: 2,
        no: 4,
    });

    monkeys.push(Monkey {
        init: vec![98, 86, 62, 76, 51, 81, 95],
        op: |old| old + 4,
        test: 13,
        yes: 5,
        no: 4,
    });

    monkeys.push(Monkey {
        init: vec![53, 95, 50, 85, 83, 72],
        op: |old| old + 5,
        test: 19,
        yes: 6,
        no: 0,
    });

    monkeys.push(Monkey {
        init: vec![59, 60, 63, 71],
        op: |old| old * 5,
        test: 11,
        yes: 5,
        no: 3,
    });

    monkeys.push(Monkey {
        init: vec![92, 65],
        op: |old| old * old,
        test: 5,
        yes: 6,
        no: 3,
    });

    monkeys.push(Monkey {
        init: vec![78],
        op: |old| old + 8,
        test: 3,
        yes: 0,
        no: 7,
    });

    monkeys.push(Monkey {
        init: vec![84, 93, 54],
        op: |old| old + 1,
        test: 17,
        yes: 2,
        no: 1,
    });

    monkeys
}

fn process(_part2: bool) -> Result<()> {
    let _ = init_monkeys();

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(args.part2) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
