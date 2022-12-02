use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    match part01() {
        Ok(score) => println!("strategy guide score: {}", score),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(score) => println!("strategy guide score: {}", score),
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
            
            score = score + match splitted_line.next() {
                Some("A") => match splitted_line.next() {
                                        Some("X") => 4,
                                        Some("Y") => 8,
                                        Some("Z") => 3,
                                        _ => panic!("unexpected input"),
                                    },
                Some("B") => match splitted_line.next() {
                                        Some("X") => 1,
                                        Some("Y") => 5,
                                        Some("Z") => 9,
                                        _ => panic!("unexpected input"),
                                    },
                Some("C") => match splitted_line.next() {
                                        Some("X") => 7,
                                        Some("Y") => 2,
                                        Some("Z") => 6,
                                        _ => panic!("unexpected input"),
                                    },
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
            
            score = score + match splitted_line.next() {
                Some("A") => match splitted_line.next() {
                                        Some("X") => 3,
                                        Some("Y") => 4,
                                        Some("Z") => 8,
                                        _ => panic!("unexpected input"),
                                    },
                Some("B") => match splitted_line.next() {
                                        Some("X") => 1,
                                        Some("Y") => 5,
                                        Some("Z") => 9,
                                        _ => panic!("unexpected input"),
                                    },
                Some("C") => match splitted_line.next() {
                                        Some("X") => 2,
                                        Some("Y") => 6,
                                        Some("Z") => 7,
                                        _ => panic!("unexpected input"),
                                    },
                _ => panic!("unexpected input"),
            }
        }
    }
    Ok(score)
}