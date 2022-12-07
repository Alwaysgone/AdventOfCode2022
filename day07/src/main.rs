use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(sum_of_dir_sizes) => println!(
            "sum of sizes of all directories with size less than 100000 part1: {}",
            sum_of_dir_sizes
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(smallest_dir_to_delete_that_allows_update) => println!(
            "size of smallest dir that allows the update part2: {}",
            smallest_dir_to_delete_that_allows_update
        ),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut current_dir = Path::new("").to_owned();
    let mut dir_sizes = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.starts_with("$ cd") {
                let mut splitted_command = line_content.split(" cd ");
                splitted_command.next();
                current_dir = match splitted_command.next() {
                    Some("..") => current_dir
                        .parent()
                        .map_or(Path::new("/").to_owned(), |p| p.to_owned()),
                    Some(dir_name) => {
                        let next_dir = current_dir.join(Path::new(dir_name));
                        if !dir_sizes.contains_key(&next_dir) {
                            dir_sizes.insert(next_dir.clone(), 0 as u32);
                        }
                        next_dir
                    }
                    None => panic!("expected dir after cd"),
                };
            } else if line_content.starts_with("$ ls") {
                // ignore
            } else if line_content.starts_with("dir") {
                // can probably be ignored
            } else {
                let mut splitted_file = line_content.split_whitespace();
                let file_size = splitted_file
                    .next()
                    .map(|file_size| file_size.parse::<u32>().unwrap())
                    .unwrap();
                for (dir, size) in dir_sizes.iter_mut() {
                    if current_dir.starts_with(dir) {
                        *size += file_size; 
                    }
                }
            }
        }
    }
    //1642503
    let mut sum_of_dir_sizes:u32 = 0;
    for (_, size) in dir_sizes.into_iter() {
        if size <= 100000 {
            sum_of_dir_sizes += size;
        }
    }
    Ok(sum_of_dir_sizes)
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut current_dir = Path::new("").to_owned();
    let mut dir_sizes = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            if line_content.starts_with("$ cd") {
                let mut splitted_command = line_content.split(" cd ");
                splitted_command.next();
                current_dir = match splitted_command.next() {
                    Some("..") => current_dir
                        .parent()
                        .map_or(Path::new("/").to_owned(), |p| p.to_owned()),
                    Some(dir_name) => {
                        let next_dir = current_dir.join(Path::new(dir_name));
                        if !dir_sizes.contains_key(&next_dir) {
                            dir_sizes.insert(next_dir.clone(), 0 as u32);
                        }
                        next_dir
                    }
                    None => panic!("expected dir after cd"),
                };
            } else if line_content.starts_with("$ ls") {
                // ignore
            } else if line_content.starts_with("dir") {
                // can probably be ignored
            } else {
                let mut splitted_file = line_content.split_whitespace();
                let file_size = splitted_file
                    .next()
                    .map(|file_size| file_size.parse::<u32>().unwrap())
                    .unwrap();
                    for (dir, size) in dir_sizes.iter_mut() {
                        if current_dir.starts_with(dir) {
                            *size += file_size; 
                        }
                    }
            }
        }
    }
    let root_file_size = dir_sizes.get(Path::new("/")).unwrap();
    let total_file_system:u32 = 70000000;
    let space_available = total_file_system - root_file_size;
    println!("space available: {}", space_available);
    let space_needed:u32 = 30000000;
    let space_to_free = space_needed - space_available;
    println!("space to free: {}", space_to_free);
    let mut smallest_dir_to_delete_that_allows_update:u32 = 0;
    for (_, size) in dir_sizes.into_iter() {
        if size >= space_to_free && (smallest_dir_to_delete_that_allows_update == 0 || size < smallest_dir_to_delete_that_allows_update){
            smallest_dir_to_delete_that_allows_update = size;
        }
    }
    Ok(smallest_dir_to_delete_that_allows_update)
}
