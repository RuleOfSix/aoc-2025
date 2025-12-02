use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("./input.txt").expect("Should be able to read input file")
}
