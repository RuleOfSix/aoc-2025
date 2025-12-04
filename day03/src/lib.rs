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
    pub fn to_string(&self) -> String {
        let mut output = String::from("");
        for digit in &self.batteries {
            output.push_str(&digit.to_string());
        }
        output
    }

    pub fn joltage(&self, num_digits: usize) -> u64 {
        let max_with_index = |(ai, acc), (i, n)| if n > acc { (i, n) } else { (ai, acc) };
        let mut digits: Vec<(usize, u32)> = Vec::new();
        digits.push(
            self.batteries
                .iter()
                .enumerate()
                .map(|(i, b)| (i, *b))
                .take_while(|(i, _)| *i <= self.batteries.len() - num_digits)
                .reduce(max_with_index)
                .unwrap(),
        );
        for n in 1..num_digits {
            digits.push(
                self.batteries
                    .iter()
                    .enumerate()
                    .skip_while(|(i, _)| *i <= digits[n - 1].0)
                    .filter(|(i, _)| self.batteries.len() - i >= num_digits - n)
                    .map(|(i, b)| (i, *b))
                    .reduce(max_with_index)
                    .unwrap(),
            );
        }
        digits.iter().enumerate().fold(0, |acc, (i, (_, n))| {
            acc + 10_u64.pow((num_digits - i - 1) as u32) * *n as u64
        })
    }
}

pub fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("Should be able to read input file")
}
