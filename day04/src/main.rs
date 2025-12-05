use day04::*;
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
    let map: RollMap = read_input();
    println!("There are {} reachable rolls", map.get_reachable().len());
}

fn run_part2() {
    let mut map: RollMap = read_input();
    let mut total = 0;
    loop {
        let reachable = map.get_reachable();
        total += match reachable.len() {
            0 => break,
            n => n,
        };
        map.remove_rolls(reachable);
    }
    println!("There are {} reachable rolls", total);
}
