use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::ops::Index;
use std::path::Path;
use std::thread::current;

fn main() {
    match part01() {
        Ok(steps_to_exit) => println!(
            "steps to exit part1: {}",
            steps_to_exit
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    // match part02() {
    //     Ok(level_of_monkey_business) => println!(
    //         "level of monkey business after 10000 rounds part2: {}",
    //         level_of_monkey_business
    //     ),
    //     Err(e) => println!("an error occurred in part02: {}", e),
    // }
}

#[derive(Debug, Clone)]
struct Route {
    visited_tiles:Vec<(usize,usize)>,
}



fn part01() -> io::Result<u32> {
    let path = Path::new("./input/sampleInput.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut height_map:Vec<String> = vec![];
    let mut start_pos:(usize,usize) = (0, 0);
    let mut exit_pos:(usize,usize) = (0, 0);
    for (current_row, line) in io::BufReader::new(file).lines().enumerate() {
        if let Ok(line_content) = line {
            if let Some(start_column) = line_content.find('S') {
                start_pos = (current_row, start_column);
            } else if let Some(exit_column) = line_content.find('E') {
                exit_pos = (current_row, exit_column);
            } 
            height_map.push(line_content.replace("S", "a").replace("E", "z"));
        }
    }
    println!("Start pos is {:?}, exit pos is {:?}", start_pos, exit_pos);
    for row in height_map.iter() {
        println!("{}", row);
    }
    let shortest_route = get_shortest_route(start_pos, exit_pos, &height_map, Route {
        visited_tiles: vec![start_pos],
    });
    println!("shortest route:");
    for tile in shortest_route.visited_tiles.iter() {
        println!("{:?}", tile);
    }
    Ok((shortest_route.visited_tiles.len() - 1) as u32)
}

fn get_shortest_route(current_pos:(usize, usize), exit_pos:(usize, usize), height_map:&Vec<String>, current_route:Route) -> Route {
    if current_pos == exit_pos {
        return current_route;
    }
    let max_column_index = height_map[0].len() - 1;
    let (current_row, current_column) = current_pos;
    let char_at = get_char_at(height_map, current_pos);
    let current_pos_value = char_at.to_digit(10).unwrap();
    let mut next_tiles:Vec<(usize, usize)> = vec![];
    if current_column > 0 {
        let new_pos = (current_row, current_column - 1);
        next_tiles.push(new_pos);
        // let new_pos_value = get_char_at(&height_map, new_pos).to_digit(10).unwrap();
        // if !current_route.visited_tiles.contains(&new_pos) && new_pos_value <= current_pos_value + 1 {
        //     let mut new_route = current_route.clone();
        //     new_route.visited_tiles.push(new_pos);
        //     let route_with_left_tile = get_shortest_route(new_pos, exit_pos, height_map, new_route);
        //     if route_with_left_tile.visited_tiles.contains(&exit_pos) {
        //         return route_with_left_tile;
        //     }
        // }
    }
    if current_column < max_column_index {
        let new_pos = (current_row, current_column + 1);
        next_tiles.push(new_pos);
        // let new_pos_value = get_char_at(&height_map, new_pos).to_digit(10).unwrap();
        // if !current_route.visited_tiles.contains(&new_pos) && new_pos_value <= current_pos_value + 1 {
        //     let mut new_route = current_route.clone();
        //     new_route.visited_tiles.push(new_pos);
        //     let route_with_left_tile = get_shortest_route(new_pos, exit_pos, height_map, new_route);
        //     if route_with_left_tile.visited_tiles.contains(&exit_pos) {
        //         return route_with_left_tile;
        //     }
        // }
    }

    if current_row > 0 {
        let new_pos = (current_row - 1, current_column);
        next_tiles.push(new_pos);
    }


    if current_row < height_map.len() - 1{
        let new_pos = (current_row + 1, current_column);
        next_tiles.push(new_pos);
    }

    for next_pos in next_tiles.into_iter() {
        let new_pos_value = get_char_at(height_map, next_pos).to_digit(10).unwrap();
        if !current_route.visited_tiles.contains(&next_pos) && new_pos_value <= current_pos_value + 1 {
            let mut new_route = current_route.clone();
            new_route.visited_tiles.push(next_pos);
            let next_route = get_shortest_route(next_pos, exit_pos, height_map, new_route);
            if next_route.visited_tiles.contains(&exit_pos) {
                return next_route;
            }
        }
    }

    todo!()
}

fn get_char_at(height_map:&Vec<String>, pos:(usize, usize)) -> char {
    let (row, column) = pos;
    height_map[row].chars().skip(column).next().unwrap().clone()
}

fn part02() -> io::Result<u64> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut current_cycle = 0;
    let mut current_value = 1;
    let mut target_cycle = 20;
    let mut signal_strength = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
        }
    }
    Ok(0)
}
