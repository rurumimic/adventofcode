use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const DAY: u8 = 2;
const URL: &str = "https://adventofcode.com/2024/day/2";
const INPUT: &str = "src/inputs/day_02.txt";

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

fn is_safe(levels: &[i32]) -> bool {
    let increase = (levels[1] - levels[0]).signum();

    for window in levels.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() > 3 || diff.signum() != increase {
            return false;
        }
    }

    true
}

fn one(reports: &[Report]) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe(&report.levels))
        .count() as i32
}

fn create_subsets(levels: &[i32]) -> impl Iterator<Item = Vec<i32>> + '_ {
    std::iter::once(levels.to_vec()).chain((0..levels.len()).map(move |index| {
        levels
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, &e)| e)
            .collect()
    }))
}

fn two(reports: &[Report]) -> i32 {
    reports
        .iter()
        .filter(|report| create_subsets(&report.levels).any(|subset| is_safe(&subset)))
        .count() as i32
}

fn main() {
    println!("Day {DAY} - {URL}\n");

    let input = env::args().nth(1).unwrap_or_else(|| INPUT.to_string());
    let f = File::open(input).expect("Can't find 'input.txt'");
    let reader = io::BufReader::new(f);

    let mut reports: Vec<Report> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Can't read a line.");

        match parse_line(&line) {
            Ok(levels) => reports.push(Report { levels }),
            Err(e) => panic!("{}", e),
        }
    }

    print!("How many reports are safe?");
    println!(" {}", one(&reports));
    print!("How many reports are now safe?");
    println!(" {}", two(&reports));
}

fn parse_line(line: &str) -> Result<Vec<i32>, String> {
    let maybe_numbers: Vec<&str> = line.trim().split_whitespace().collect();
    if maybe_numbers.len() < 2 {
        return Err(format!("Invalid line format: {}", line));
    }

    maybe_numbers
        .into_iter()
        .map(|n| n.parse::<i32>().map_err(|e| format!("Can't parse: {}", e)))
        .collect()
}

/*

fn create_subsets(levels: &[i32]) -> Vec<Vec<i32>> {
    let mut subsets = Vec::with_capacity(levels.len() + 1);
    subsets.push(levels.to_vec());

    for index in 0..levels.len() {
        let subset: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, &e)| e)
            .collect();
        subsets.push(subset);
    }

    subsets
}

fn two(reports: &[Report]) -> i32 {
    let mut safe = 0;

    for report in reports {
        let subsets: Vec<Vec<i32>> = create_subsets(&report.levels);

        for levels in subsets {
            if is_safe(&levels) {
                safe += 1;
                break;
            }
        }
    }

    safe
}

*/
