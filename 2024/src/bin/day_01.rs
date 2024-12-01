use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

static DAY: u8 = 1;
static URL: &str = "https://adventofcode.com/2024/day/1";
static INPUT: &str = "src/inputs/day_01.txt";

struct Locations {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn one(locations: &Locations) -> i32 {
    let mut distance = 0;

    let mut left = locations.left.clone();
    let mut right = locations.right.clone();

    left.sort();
    right.sort();

    // 1. Using a for loop
    let iter = left.iter().zip(right.iter());
    for (l, r) in iter {
        distance += (l - r).abs();
    }

    // 2. Using map
    distance = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    distance
}

#[allow(unused_assignments)] // for similarity
fn two(locations: &Locations) -> i32 {
    let mut similarity = 0;

    let left = &locations.left;
    let right = &locations.right;

    // 1. Using a for loop
    for l in left.iter() {
        let count = right.iter().filter(|r| *r == l).count() as i32;
        similarity += l * count;
    }

    // 2. Using map
    similarity = left
        .iter()
        .map(|l| l * right.iter().filter(|r| *r == l).count() as i32)
        .sum::<i32>();

    // 3. Using a HashMap
    let mut right_counts = HashMap::new();
    for &r in right.iter() {
        *right_counts.entry(r).or_insert(0) += 1;
    }
    similarity = left
        .iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0))
        .sum();

    similarity
}

fn main() {
    println!("Day {DAY} - {URL}\n");

    let input = env::args().nth(1).unwrap_or_else(|| INPUT.to_string());
    let f = File::open(input).expect("Can't find 'input.txt'");
    let reader = io::BufReader::new(f);

    let mut locations = Locations {
        left: vec![],
        right: vec![],
    };

    for line in reader.lines() {
        let line = line.expect("Can't read a line.");

        match parse_line(&line) {
            Ok((left, right)) => {
                locations.left.push(left);
                locations.right.push(right);
            }
            Err(e) => panic!("{}", e),
        }
    }

    print!("What is the total distance between your lists?");
    println!(" {}", one(&locations));
    print!("What is their similarity score?");
    println!(" {}", two(&locations));
}

fn parse_line(line: &str) -> Result<(i32, i32), String> {
    let maybe_numbers: Vec<&str> = line.trim().split_whitespace().collect();
    if maybe_numbers.len() != 2 {
        return Err(format!("Invalid line format: {}", line));
    }

    let left = maybe_numbers[0]
        .parse::<i32>()
        .map_err(|_| format!("Can't parse string: {}", maybe_numbers[0]))?;
    let right = maybe_numbers[1]
        .parse::<i32>()
        .map_err(|_| format!("Can't parse string: {}", maybe_numbers[0]))?;

    Ok((left, right))
}
