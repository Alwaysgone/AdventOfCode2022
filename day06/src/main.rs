use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(start_sequence_index) => println!("number of characters before starting sequence part1: {}", start_sequence_index),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(start_message_index) => println!("number of characters before starting sequence part2: {}", start_message_index),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut start_sequence_index:u32 = 0;
    let message_size:u32 = 4;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            for i in 0..line_content.len() {
                // pretty inefficient but looks cool and it's rust so...
                let mut current_chars:Vec<char> = line_content.chars().skip(i).take(message_size as usize).collect();
                current_chars.sort();
                current_chars.dedup();
                if current_chars.len() as u32 == message_size {
                    start_sequence_index = i as u32 + message_size;
                    break;
                }
            }
        }
    }

    Ok(start_sequence_index)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut start_message_index = 0;
    let message_size:u32 = 14;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            for i in 0..line_content.len() {
                let mut current_chars:Vec<char> = line_content.chars().skip(i).take(message_size as usize).collect();
                current_chars.sort();
                current_chars.dedup();
                if current_chars.len() as u32 == message_size {
                    start_message_index = i as u32 + message_size;
                    break;
                }
            }
        }
    }


    Ok(start_message_index)
}
