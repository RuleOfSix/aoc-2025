use day02::*;
use std::env;

enum Part {
    One,
    Two,
}

impl Part {
    fn build(p: &str) -> Option<Self> {
        match p {
            "p1" => Some(Self::One),
            "p2" => Some(Self::Two),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = Part::build(args.get(1).unwrap_or_else(|| usage(&args)));
    run(part.unwrap_or_else(|| usage(&args)));
}

fn usage(args: &[String]) -> ! {
    println!(
        "Usage: {} [p1|p2]",
        args.get(0)
            .expect("Should be able to access zeroth argument")
    );
    std::process::exit(1);
}

fn run(part: Part) {
    let input = read_input();
    let ranges: Vec<Range> = input
        .trim()
        .split(',')
        .map(|r| r.split('-').collect::<Vec<_>>())
        .map(|r| (r[0], r[1]))
        .map(|(l, h)| (l.parse(), h.parse()))
        .map(|(l, h)| (l.expect("Low invalid"), h.expect("High invalid")))
        .map(|(l, h)| Range::new(l, h))
        .collect();

    let mut invalids: Vec<i64> = Vec::new();
    let pk = match part {
        Part::One => PatternKind::Twice,
        Part::Two => PatternKind::Any,
    };
    for range in ranges {
        invalids.append(&mut range.invalid_ids(pk.clone()));
    }

    println!(
        "The sum of invalid ids is {}",
        invalids.iter().fold(0, |acc, id| acc + *id)
    );
}
