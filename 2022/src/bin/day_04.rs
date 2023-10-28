use std::fs::File;
use std::io;

static DAY: u8 = 4;
static URL: &str = "https://adventofcode.com/2022/day/4";
static INPUT: &str = "src/inputs/day_04.txt";

fn one(items: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for item in items {
        let sections: Vec<u8> = item
            .split(',')
            .flat_map(|line| line.split('-').collect::<Vec<&str>>())
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        if sections[0] <= sections[2] && sections[1] >= sections[3] {
            sum += 1;
        } else if sections[0] >= sections[2] && sections[1] <= sections[3]  {
            sum += 1;
        } else {
            continue;
        }

    }

    sum
}

fn two(items: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for item in items {
        let sections: Vec<u8> = item
            .split(',')
            .flat_map(|line| line.split('-').collect::<Vec<&str>>())
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        if sections[0] <= sections[2] && sections[1] >= sections[2] {
            sum += 1;
        } else if sections[0] >= sections[2] && sections[0] <= sections[3]  {
            sum += 1;
        } else {
            continue;
        }

    }

    sum
}

fn main() -> Result<(), io::Error> {
    println!("Day {DAY} - {URL}\n");

    let file = File::open(INPUT)?;
    let binding = io::read_to_string(file)?;
    let items: Vec<&str> = binding.split('\n').filter(|x| x.len() > 0).collect();

    print!("In how many assignment pairs does one range fully contain the other?");
    println!(" {}", one(&items));
    print!("In how many assignment pairs do the ranges overlap?");
    println!(" {}", two(&items));

    Ok(())
}

