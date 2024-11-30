use std::fs::File;
use std::io::{self};

static DAY: u8 = 1;
static URL: &str = "https://adventofcode.com/2024/day/1";
static INPUT: &str = "src/inputs/day_01.txt";

fn one() -> u32 {
    0
}

fn two() -> u32 {
    0
}

fn main() {
    println!("Day {DAY} - {URL}\n");

    let f = File::open(INPUT).expect("Can't find 'input.txt'");
    let mut _reader = io::BufReader::new(f);

    _ = one();
    _ = two();
}
