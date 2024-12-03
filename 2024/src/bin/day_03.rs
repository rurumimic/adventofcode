use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const DAY: u8 = 3;
const URL: &str = "https://adventofcode.com/2024/day/3";
const INPUT: &str = "src/inputs/day_03.txt";

enum Either<L, R> {
    Left(L),
    Right(R),
}

fn parse(c: char) -> Either<u16, char> {
    match c.to_string().parse::<u16>() {
        Ok(n) => Either::Left(n),
        Err(_) => Either::Right(c),
    }
}

fn one(lines: &[String]) -> i32 {
    let mut sum = 0;

    let expr = ['m', 'u', 'l', '(', '0', ',', '0', ')'];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut position = 0;
        let mut number_len = 0;
        let mut 
        for i in 0..chars.len() {
            match parse(chars[i]) {
                Either::Left(n) => {
                    if position == 4 || position == 6 {
                        if number_len < 3 {
                            number_len -= 1;
                        } else {
                            break;
                        }
                    } else {
                        position = 0;
                        number_len = 3;
                        break;
                    }
                }
                Either::Right(c) => {}
            }
            if chars[i] == expr[position] {
                position += 1;
            }
        }
    }
    sum
}

fn two(_lines: &[String]) -> i32 {
    0
}

fn main() {
    println!("Day {DAY} - {URL}\n");

    let input = env::args().nth(1).unwrap_or_else(|| INPUT.to_string());
    let f = File::open(input).expect("Can't find 'input.txt'");
    let reader = io::BufReader::new(f);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();

    print!("What do you get if you add up all of the results of the multiplications?");
    println!(" {}", one(&lines));
    print!("?");
    println!(" {}", two(&lines));
}
