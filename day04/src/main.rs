use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let s = "thing-thing2,otherthing1-otherthing2";
    match part01() {
        Ok(sum_of_fully_contained_pairs) => println!(
            "sum of fully contained assignment pairs part1: {}",
            sum_of_fully_contained_pairs
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(sum_of_overlapping_pairs) => println!("sum of fully contained assignment pairs part2: {}", sum_of_overlapping_pairs),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut sum_of_fully_contained_pairs: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split(&[',', '-'][..]);
            let (left_lower, left_upper, right_lower, right_upper) = (
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
            );

            if (left_lower <= right_lower && left_upper >= right_upper)
                || (right_lower <= left_lower && right_upper >= left_upper)
            {
                sum_of_fully_contained_pairs = sum_of_fully_contained_pairs + 1;
            }
        }
    }
    Ok(sum_of_fully_contained_pairs)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut sum_of_overlapping_pairs: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split(&[',', '-'][..]);
            let (left_lower, left_upper, right_lower, right_upper) = (
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
                splitted_line
                    .next()
                    .map(|s| s.parse::<u32>().unwrap())
                    .unwrap(),
            );

            if left_lower <= right_upper && left_upper >= right_lower
            {
                sum_of_overlapping_pairs = sum_of_overlapping_pairs + 1;
            }
        }
    }
    Ok(sum_of_overlapping_pairs)
}
