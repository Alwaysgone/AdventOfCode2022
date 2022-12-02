use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    match part01() {
        Ok(score) => println!("strategy guide score part1: {}", score),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(score) => println!("strategy guide score part2: {}", score),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    /*
        A = Rock
        B = Paper
        C = Scissor

        X = Rock
        Y = Paper
        Z = Scissor

        lose = 0 points
        draw = 3 points
        win = 6 points
    */
    let mut score:u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split_whitespace();
            let round = (splitted_line.next(), splitted_line.next());
            score = score + match round {
                (Some("A"), Some("X")) => 4,
                (Some("A"), Some("Y")) => 8,
                (Some("A"), Some("Z")) => 3,
                (Some("B"), Some("X")) => 1,
                (Some("B"), Some("Y")) => 5,
                (Some("B"), Some("Z")) => 9,
                (Some("C"), Some("X")) => 7,
                (Some("C"), Some("Y")) => 2,
                (Some("C"), Some("Z")) => 6,
                _ => panic!("unexpected input"),
            }
        }
    }
    Ok(score)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    /*
        A = Rock
        B = Paper
        C = Scissor

        X = need to lose
        Y = need to end in a draw
        Z = need to win

        lose = 0 points
        draw = 3 points
        win = 6 points
    */
    let mut score:u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split_whitespace();
            let round = (splitted_line.next(), splitted_line.next());
            score = score + match round {
                (Some("A"), Some("X")) => 3,
                (Some("A"), Some("Y")) => 4,
                (Some("A"), Some("Z")) => 8,
                (Some("B"), Some("X")) => 1,
                (Some("B"), Some("Y")) => 5,
                (Some("B"), Some("Z")) => 9,
                (Some("C"), Some("X")) => 2,
                (Some("C"), Some("Y")) => 6,
                (Some("C"), Some("Z")) => 7,
                _ => panic!("unexpected input"),
            }
        }
    }
    Ok(score)
}