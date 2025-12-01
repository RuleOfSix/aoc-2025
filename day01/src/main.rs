use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Dial {
    location: i16,
}

impl Dial {
    fn new() -> Dial {
        Dial { location: 50 }
    }

    fn turn(&mut self, dir: &Direction, amount: i16, count: &mut i32) {
        if amount < 0 {
            return;
        }
        match *dir {
            Direction::Left => {
                self.shift_by_amount(-amount, count);
            }
            Direction::Right => {
                self.shift_by_amount(amount, count);
            }
        }
    }

    fn shift_by_amount(&mut self, amount: i16, count: &mut i32) {
        let was_zero = self.location == 0;
        self.location += amount;
        let overflow = !self.in_range();
        while !self.in_range() {
            self.location += self.shift_value();
            *count += 1;
        }
        if overflow {
            *count -= if was_zero && amount < 0 { 1 } else { 0 };
            *count -= if self.location == 0 && amount > 0 {
                1
            } else {
                0
            };
        }
    }

    fn shift_value(&self) -> i16 {
        if self.location < 0 {
            100
        } else if self.location > 99 {
            -100
        } else {
            0
        }
    }

    fn in_range(&self) -> bool {
        self.location >= 0 && self.location < 100
    }
}

fn main() {
    let mut input_file = File::open("input.txt").expect("Should be able to open input data");
    let mut input_str = String::new();
    let mut dial = Dial::new();
    let mut count: i32 = 0;
    input_file
        .read_to_string(&mut input_str)
        .expect("Should be able to read input data");

    for line in input_str.lines() {
        let dir = match line.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => break,
        };
        let amount: i16 = line
            .get(1..)
            .and_then(|a| a.parse().ok())
            .expect("Should be able to parse rotation amount");
        dial.turn(&dir, amount, &mut count);

        if dial.location == 0 {
            count += 1;
        }

        println!(
            "Turning {dir:?} by {amount}...\tDial: {:2}\tcount: {count}",
            dial.location
        );
    }

    println!("\nThe password is {count}");
}
