use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    match part01() {
        Ok(top_crates) => println!("top crates part1: {}", top_crates),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(top_crates) => println!("top crates part2: {}", top_crates),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn move_crate(mut crate_stacks:Vec<VecDeque<char>>, from_index:usize, to_index:usize, number_of_crates:usize) -> Vec<VecDeque<char>> {
    for _ in 0..number_of_crates {
        let crate_to_move = crate_stacks.get_mut(from_index).unwrap().pop_back().unwrap();
        crate_stacks.get_mut(to_index).unwrap().push_back(crate_to_move);
    }
    crate_stacks
}

fn move_all_crates_at_once(mut crate_stacks:Vec<VecDeque<char>>, from_index:usize, to_index:usize, number_of_crates:usize) -> Vec<VecDeque<char>> {
    let mut crate_buffer = VecDeque::new();
    for _ in 0..number_of_crates {
        let crate_to_move = crate_stacks.get_mut(from_index).unwrap().pop_back().unwrap();
        crate_buffer.push_front(crate_to_move);
    }
    for _ in 0..number_of_crates {
        crate_stacks.get_mut(to_index).unwrap().push_back(crate_buffer.pop_front().unwrap());
    }
    crate_stacks
}

fn part01() -> io::Result<String> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut top_crates: String = "".to_string();
    let mut crate_stacks = vec![];
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.starts_with(" 1") {
                // can ignore since indeces are implicit
            } else if line_content.starts_with("move"){
                //move 5 from 8 to 2
                let mut splitted_line = line_content.split_whitespace();
                // move
                splitted_line.next();
                // number of crates
                let number_of_crates = splitted_line.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
                // from
                splitted_line.next();
                // from_index
                let from_index = splitted_line.next().map(|s| s.parse::<usize>().unwrap() - 1).unwrap();
                // to
                splitted_line.next();
                // to_index
                let to_index = splitted_line.next().map(|s| s.parse::<usize>().unwrap() - 1).unwrap();
                crate_stacks = move_crate(crate_stacks, from_index, to_index, number_of_crates);
            } else {
                if crate_stacks.is_empty() {
                    for _ in 0..((line_content.len() + 1) / 4) {
                        let stack:VecDeque::<char> = VecDeque::new();
                        crate_stacks.push(stack);
                    }
                }
                let mut next_char_is_crate = false;
                for (i, c)  in line_content.chars().enumerate() {
                    if next_char_is_crate {
                        next_char_is_crate = false;
                        let current_stack_index = (i - 1) / 4;
                        match crate_stacks.get_mut(current_stack_index) {
                            Some(stack) => stack.push_front(c),
                            None => panic!("Expected stack with index {}", current_stack_index),
                        }
                    }
                    if c == '[' {
                        next_char_is_crate = true;
                    }
                }
            }
        }
    }

    for (i, mut stack) in crate_stacks.into_iter().enumerate() {
        // let c:char = *stack.first().unwrap();
        let first_element = stack.pop_back().unwrap();
        println!("First stack element {}", first_element);
        println!("Stack {}: {:?}", i, stack);
        top_crates.push(first_element);
        println!("Stack without top crate {}: {:?}", i, stack);
    }

    Ok(top_crates)
}

fn part02() -> io::Result<String> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut top_crates: String = "".to_string();
    let mut crate_stacks = vec![];
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.starts_with(" 1") {
                // can ignore since indeces are implicit
            } else if line_content.starts_with("move"){
                //move 5 from 8 to 2
                let mut splitted_line = line_content.split_whitespace();
                // move
                splitted_line.next();
                // number of crates
                let number_of_crates = splitted_line.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
                // from
                splitted_line.next();
                // from_index
                let from_index = splitted_line.next().map(|s| s.parse::<usize>().unwrap() - 1).unwrap();
                // to
                splitted_line.next();
                // to_index
                let to_index = splitted_line.next().map(|s| s.parse::<usize>().unwrap() - 1).unwrap();
                crate_stacks = move_all_crates_at_once(crate_stacks, from_index, to_index, number_of_crates);

            } else {
                if crate_stacks.is_empty() {
                    for _ in 0..((line_content.len() + 1) / 4) {
                        let stack:VecDeque::<char> = VecDeque::new();
                        crate_stacks.push(stack);
                    }
                }
                let mut next_char_is_crate = false;
                for (i, c)  in line_content.chars().enumerate() {
                    if next_char_is_crate {
                        next_char_is_crate = false;
                        let current_stack_index = (i - 1) / 4;
                        match crate_stacks.get_mut(current_stack_index) {
                            Some(stack) => stack.push_front(c),
                            None => panic!("Expected stack with index {}", current_stack_index),
                        }
                    }
                    if c == '[' {
                        next_char_is_crate = true;
                    }
                }
            }
        }
    }

    Ok(top_crates)
}
