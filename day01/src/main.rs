use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    match part01() {
        Ok(max_calories) => println!("max calories: {}", max_calories),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(sum_of_top_three_elves) => println!("sum of top three elves calories: {}", sum_of_top_three_elves),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };

    let mut max_calories:u32 = 0;
    let mut current_calories:u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.is_empty() {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            } else {
                current_calories = current_calories + line_content.parse::<u32>().unwrap();
            }
        }
    }
    Ok(max_calories)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut top_elves = vec![];

    let mut current_calories:u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.is_empty() {
                top_elves = rate_calories(top_elves, current_calories);
                current_calories = 0;
            } else {
                current_calories = current_calories + line_content.parse::<u32>().unwrap();
            }
        }
    }
    Ok(top_elves.into_iter().sum())
}

fn rate_calories(mut elves:Vec<u32>, calories: u32) -> Vec<u32> {
    elves.push(calories);
    if elves.len() > 3 {
        match elves.iter().min() {
            Some(min_calories) => {
                elves = elves.clone().into_iter()
                .filter(|c| c > min_calories)
                .collect();
            },
            _ => {},
        }
    }
    elves
}