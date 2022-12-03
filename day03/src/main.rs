use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(score) => println!("sum of all priorities part1: {}", score),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(score) => println!("sum of all priorities part2: {}", score),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn to_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else if c.is_ascii_uppercase() {
        c as u32 - 38
    } else {
        panic!("cannot convert {} to priority", c);
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut sum_priorities: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let (left_comparment, right_compartment) =
                line_content.split_at(line_content.len() / 2);
            for c in left_comparment.chars() {
                if right_compartment.contains(c) {
                    sum_priorities = sum_priorities + to_priority(c);
                    break;
                }
            }
        }
    }
    Ok(sum_priorities)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut sum_priorities: u32 = 0;
    for chunk in lines.chunks(3) {
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                sum_priorities = sum_priorities + to_priority(c);
                break;
            }
        }
    }
    Ok(sum_priorities)
}
