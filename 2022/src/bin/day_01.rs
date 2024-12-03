use std::fs::File;
use std::io::{self, BufRead};

const DAY: u8 = 1;
const URL: &str = "https://adventofcode.com/2022/day/1";
const INPUT: &str = "src/inputs/day_01.txt";

fn one(calories: Vec<Vec<u32>>) -> u32 {
    calories
        .iter()
        .map(|elf| elf.iter().sum())
        .reduce(u32::max)
        .unwrap()
}

fn two(calories: Vec<Vec<u32>>) -> u32 {
    let mut elves = calories
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();

    elves.sort_by(|x, y| y.cmp(x));
    elves.iter().take(3).sum()
}

fn main() {
    println!("Day {DAY} - {URL}\n");

    let f = File::open(INPUT).expect("Can't find 'input.txt'");
    let mut reader = io::BufReader::new(f);

    let mut calories: Vec<Vec<u32>> = vec![];
    let mut elf: Vec<u32> = vec![];
    let mut line = String::new();

    loop {
        let nread = reader.read_line(&mut line).expect("Can't read a line.");
        if nread == 0 {
            break;
        }

        let maybe_number = line.trim();

        if maybe_number.len() > 0 {
            let number: u32 = maybe_number.parse().expect("Can't parse string");
            elf.push(number);
        } else {
            calories.push(elf.clone());
            elf.clear();
        }

        line.clear();
    }

    print!("How many total Calories is that Elf carrying? ");
    println!("{}", one(calories.clone()));

    print!("How many Calories are those Elves carrying in total? ");
    println!("{}", two(calories));
}
