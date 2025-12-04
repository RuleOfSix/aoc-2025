use day03::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let print_usage = || {
        println!(
            "Usage: {} [p1|p2]",
            args.get(0)
                .expect("Should be able to access zeroth argument")
        );
        std::process::exit(1)
    };
    let part = args.get(1).unwrap_or_else(print_usage);
    if part == "p1" {
        run_part1();
    } else if part == "p2" {
        run_part2();
    } else {
        print_usage();
    }
}

fn run_part1() {
    let banks: Vec<BatteryBank> = read_input()
        .split("\n")
        .filter(|b| !b.is_empty())
        .map(|b| BatteryBank::from(b))
        .collect();
    let mut total = 0;
    for bank in banks {
        total += bank.joltage(2);
    }
    println!("Total joltage: {total}");
}

fn run_part2() {
    let banks: Vec<BatteryBank> = read_input()
        .split("\n")
        .filter(|b| !b.is_empty())
        .map(|b| BatteryBank::from(b))
        .collect();
    let mut total = 0;
    for bank in banks {
        total += bank.joltage(12);
    }
    println!("Total joltage: {total}");
}
