use std::collections::HashSet;
use std::fs::File;
use std::io;

static DAY: u8 = 3;
static URL: &str = "https://adventofcode.com/2022/day/3";
static INPUT: &str = "src/inputs/day_03.txt";

static LOWER_A: u32 = 'a' as u32;

fn one(items: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for item in items {
        let length = item.len();

        let first: HashSet<char> = HashSet::from_iter(item[..length / 2].chars());
        let second: HashSet<char> = HashSet::from_iter(item[length / 2..].chars());

        let share = &first & &second;

        let letter = *share.iter().take(1).next().unwrap();
        let number = letter as u32;

        let priority = match number < LOWER_A {
            true => number - 'A' as u32 + 27,
            false => number - 'a' as u32 + 1,
        };

        sum += priority;
    }

    sum
}

fn to_set(str: &str) -> HashSet<char> {
    HashSet::from_iter(str.chars())
}

fn two(items: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    let groups: Vec<_> = items
        .chunks(3)
        .map(|chunk| &(&to_set(&chunk[0]) & &to_set(&chunk[1])) & &to_set(&chunk[2]))
        .map(|s| s.into_iter().take(1).next().unwrap())
        .collect();

    for letter in groups {
        let number = letter as u32;

        let priority = match number < LOWER_A {
            true => number - 'A' as u32 + 27,
            false => number - 'a' as u32 + 1,
        };

        sum += priority;
    }

    sum
}

fn main() -> Result<(), io::Error> {
    println!("Day {DAY} - {URL}\n");

    let file = File::open(INPUT)?;
    let binding = io::read_to_string(file)?;
    let items: Vec<&str> = binding.split('\n').filter(|x| x.len() > 0).collect();

    print!("What is the sum of the priorities of those item types?");
    println!(" {}", one(&items));
    print!("What is the sum of the priorities of those item types?");
    println!(" {}", two(&items));

    Ok(())
}

