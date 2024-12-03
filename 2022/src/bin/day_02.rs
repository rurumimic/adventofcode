use std::fs::File;
use std::io;

const DAY: u8 = 2;
const URL: &str = "https://adventofcode.com/2022/day/2";
const INPUT: &str = "src/inputs/day_02.txt";

#[derive(Debug, PartialEq)]
enum Round {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq)]
enum Kind {
    Rock,
    Paper,
    Scissors,
}

struct Choice {
    kind: Kind,
}

impl Choice {
    fn new(a: char) -> Choice {
        match a {
            'A' | 'X' => Choice { kind: Kind::Rock },
            'B' | 'Y' => Choice { kind: Kind::Paper },
            'C' | 'Z' => Choice {
                kind: Kind::Scissors,
            },
            _ => panic!("Wrong Input"),
        }
    }

    fn compare(&self, opponent: &Choice) -> Round {
        if self.kind == opponent.kind {
            return Round::Draw;
        }

        if (self.kind == Kind::Rock && opponent.kind == Kind::Scissors)
            || (self.kind == Kind::Paper && opponent.kind == Kind::Rock)
            || (self.kind == Kind::Scissors && opponent.kind == Kind::Paper)
        {
            return Round::Win;
        }

        Round::Loss
    }
}

fn one(rounds: &Vec<Vec<&str>>) -> i32 {
    let mut points = 0;

    for line in rounds {
        if line.len() < 2 {
            break;
        }

        let elf = Choice::new(line[0].chars().next().unwrap());
        let me = Choice::new(line[1].chars().next().unwrap());

        let res = me.compare(&elf);

        match res {
            Round::Win => points += 6,
            Round::Draw => points += 3,
            Round::Loss => {}
        };

        match me.kind {
            Kind::Rock => points += 1,
            Kind::Paper => points += 2,
            Kind::Scissors => points += 3,
        }
    }

    points
}

fn two(rounds: &Vec<Vec<&str>>) -> i32 {
    let mut points = 0;

    for line in rounds {
        if line.len() < 2 {
            break;
        }

        let elf = Choice::new(line[0].chars().next().unwrap());
        let me = Choice::new(line[1].chars().next().unwrap());

        match me.kind {
            Kind::Rock => {
                points += 0;
                match elf.kind {
                    Kind::Rock => points += 3,
                    Kind::Paper => points += 1,
                    Kind::Scissors => points += 2,
                }
            }
            Kind::Paper => {
                points += 3;
                match elf.kind {
                    Kind::Rock => points += 1,
                    Kind::Paper => points += 2,
                    Kind::Scissors => points += 3,
                }
            }
            Kind::Scissors => {
                points += 6;
                match elf.kind {
                    Kind::Rock => points += 2,
                    Kind::Paper => points += 3,
                    Kind::Scissors => points += 1,
                }
            }
        }
    }

    points
}

fn main() -> Result<(), io::Error> {
    println!("Day {DAY} - {URL}\n");

    let file = File::open(INPUT)?;
    let binding = io::read_to_string(file)?;
    let rounds: Vec<Vec<&str>> = binding
        .split('\n')
        .map(|line| line.split(' ').collect())
        .collect();

    print!("What would your total score be if everything goes exactly according to your strategy guide?");
    println!(" {}", one(&rounds));

    print!("What would your total score be if everything goes exactly according to your strategy guide?");
    println!(" {}", two(&rounds));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        assert_eq!(Choice::new('A').compare(&Choice::new('X')), Round::Draw);
        assert_eq!(Choice::new('A').compare(&Choice::new('Y')), Round::Loss);
        assert_eq!(Choice::new('A').compare(&Choice::new('Z')), Round::Win);
    }
}
