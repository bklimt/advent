use std::collections::VecDeque;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    debug: bool,

    #[arg(long)]
    test: bool,

    #[arg(long)]
    part2: bool,
}

struct Monkey {
    items: VecDeque<i64>,
    op: fn(i64) -> i64,
    test: i64,
    yes: usize,
    no: usize,
    inspected: i64,
}

fn test_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    monkeys.push(Monkey {
        items: vec![79, 98].into(),
        op: |old| old * 19,
        test: 23,
        yes: 2,
        no: 3,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![54, 65, 75, 74].into(),
        op: |old| old + 6,
        test: 19,
        yes: 2,
        no: 0,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![79, 60, 97].into(),
        op: |old| old * old,
        test: 13,
        yes: 1,
        no: 3,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![74].into(),
        op: |old| old + 3,
        test: 17,
        yes: 0,
        no: 1,
        inspected: 0,
    });

    monkeys
}

fn init_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    monkeys.push(Monkey {
        items: vec![62, 92, 50, 63, 62, 93, 73, 50].into(),
        op: |old| old * 7,
        test: 2,
        yes: 7,
        no: 1,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![51, 97, 74, 84, 99].into(),
        op: |old| old + 3,
        test: 7,
        yes: 2,
        no: 4,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![98, 86, 62, 76, 51, 81, 95].into(),
        op: |old| old + 4,
        test: 13,
        yes: 5,
        no: 4,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![53, 95, 50, 85, 83, 72].into(),
        op: |old| old + 5,
        test: 19,
        yes: 6,
        no: 0,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![59, 60, 63, 71].into(),
        op: |old| old * 5,
        test: 11,
        yes: 5,
        no: 3,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![92, 65].into(),
        op: |old| old * old,
        test: 5,
        yes: 6,
        no: 3,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![78].into(),
        op: |old| old + 8,
        test: 3,
        yes: 0,
        no: 7,
        inspected: 0,
    });

    monkeys.push(Monkey {
        items: vec![84, 93, 54].into(),
        op: |old| old + 1,
        test: 17,
        yes: 2,
        no: 1,
        inspected: 0,
    });

    monkeys
}

fn process(args: Args) -> Result<()> {
    let mut monkeys = if args.test {
        test_monkeys()
    } else {
        init_monkeys()
    };

    let rounds = if args.part2 { 10000 } else { 20 };
    for round in 1..=rounds {
        for i in 0..monkeys.len() {
            if args.debug {
                println!("");
                println!("Monkey {}", i);
            }

            while let Some(item) = monkeys[i].items.pop_front() {
                if args.debug {
                    println!("  Monkey inspects an item with a worry level of {}.", item);
                }
                monkeys[i].inspected += 1;
                let worry = (monkeys[i].op)(item);
                if args.debug {
                    println!("    Worry level is op'd to {}.", worry);
                }
                let worry = if args.part2 { worry } else { worry / 3 };
                if args.debug && !args.part2 {
                    println!(
                        "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                        worry
                    );
                }
                let new_monkey = if worry % monkeys[i].test == 0 {
                    if args.debug {
                        println!(
                            "    Current worry level is divisible by {}.",
                            monkeys[i].test
                        );
                    }
                    monkeys[i].yes
                } else {
                    if args.debug {
                        println!(
                            "    Current worry level is not divisible by {}.",
                            monkeys[i].test
                        );
                    }
                    monkeys[i].no
                };
                if args.debug {
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        worry, new_monkey
                    );
                }
                monkeys[new_monkey].items.push_back(worry);
            }
        }
        if args.debug {
            println!("");
            println!("Round {}", round);
            for (i, monkey) in monkeys.iter().enumerate() {
                println!("Monkey {}: {:?}", i, monkey.items);
            }
        }
    }

    let mut inspected = Vec::new();
    println!("");
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {}", i, monkey.inspected);
        inspected.push(monkey.inspected);
    }
    println!("");

    inspected.sort();
    let mut inspected = inspected.iter().rev();
    let n1 = inspected.next().unwrap();
    let n2 = inspected.next().unwrap();
    let ans = n1 * n2;
    println!("ans = {}", ans);

    Ok(())
}

fn main() {
    let args = Args::parse();
    match process(args) {
        Ok(_) => (),
        Err(error) => panic!("{:?}", error),
    };
}
