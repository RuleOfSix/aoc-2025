use std::fs;

pub struct BatteryBank {
    batteries: Vec<u32>,
}

impl From<Vec<u32>> for BatteryBank {
    fn from(batteries: Vec<u32>) -> Self {
        BatteryBank { batteries }
    }
}

impl From<&str> for BatteryBank {
    fn from(batteries: &str) -> Self {
        BatteryBank {
            batteries: batteries.chars().map(|c| c.to_digit(10).unwrap()).collect(),
        }
    }
}

impl BatteryBank {
    pub fn joltage(&self) -> u32 {
        let tup_max = |(ai, acc), (i, n)| if n > acc { (i, n) } else { (ai, acc) };
        let (d1_index, d1) = self
            .batteries
            .split_last()
            .unwrap()
            .1
            .iter()
            .enumerate()
            .reduce(tup_max)
            .unwrap();
        let d2 = self
            .batteries
            .split_at(d1_index)
            .1
            .iter()
            .skip(1)
            .reduce(|acc, b| acc.max(b))
            .unwrap();
        10 * d1 + d2
    }
}

pub fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("Should be able to read input file")
}
