use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(number_of_tiles_tail_visited) => println!(
            "number of tiles the tail visited at least once part1: {}",
            number_of_tiles_tail_visited
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(number_of_tiles_tail_visited) => println!(
            "number of tiles the tail visited at least once part2: {}",
            number_of_tiles_tail_visited
        ),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn is_knot_adjacent(knot1: &(i32, i32), knot2: &(i32, i32)) -> bool {
    (knot1.0 - knot2.0).abs() <= 1 && (knot1.1 - knot2.1).abs() <= 1
}

fn move_knot(source_position: (i32, i32), target_position: &(i32, i32)) -> (i32, i32) {
    let directional_vector = (target_position.0 - source_position.0, target_position.1 - source_position.1);
    // this only works because it is already known that the tail is not adjacent to the head
    // so a distance of 1 on the x axis must mean that the head is either -2 or 2 on the y axis
    // away from the head which entails a diagonal move
    let x_movement = match directional_vector.0 {
        2 => 1,
        -2 => -1,
        1 => 1,
        -1 => -1,
        _ => 0,
    };
    let y_movement = match directional_vector.1 {
        2 => 1,
        -2 => -1,
        1 => 1,
        -1 => -1,
        _ => 0,
    };
    (source_position.0 + x_movement, source_position.1 + y_movement)
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut tiles_tail_visited = vec![];
    let mut head_knot = (1000, 1000);
    let mut tail_knot = (1000, 1000);
    tiles_tail_visited.push(tail_knot);
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split_whitespace();
            let move_function = match splitted_line.next() {
                Some("R") => |head:(i32, i32)| (head.0 + 1, head.1),
                Some("L") => |head:(i32, i32)| (head.0 - 1, head.1),
                Some("U") => |head:(i32, i32)| (head.0, head.1 - 1),
                Some("D") => |head:(i32, i32)| (head.0, head.1 + 1),
                Some(_) => panic!("unrecognized direction"),
                None => panic!("expected direction"),
            };
            let number_of_times = splitted_line.next().map(|v|v.parse::<u32>().unwrap()).unwrap();
            for _ in 0..number_of_times {
                head_knot = move_function(head_knot);
                if !is_knot_adjacent(&head_knot, &tail_knot) {
                    tail_knot = move_knot(tail_knot, &head_knot);
                    if !tiles_tail_visited.contains(&tail_knot) {
                        tiles_tail_visited.push(tail_knot);
                    }
                }
            }
        }
    }
    Ok(tiles_tail_visited.len() as u32)
}


fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut tiles_tail_visited = vec![];
    let mut head_knot = (1000, 1000);
    let mut knots = vec![(1000, 1000); 9];
    tiles_tail_visited.push(knots[8]);
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split_whitespace();
            let move_function = match splitted_line.next() {
                Some("R") => |head:(i32, i32)| (head.0 + 1, head.1),
                Some("L") => |head:(i32, i32)| (head.0 - 1, head.1),
                Some("U") => |head:(i32, i32)| (head.0, head.1 - 1),
                Some("D") => |head:(i32, i32)| (head.0, head.1 + 1),
                Some(_) => panic!("unrecognized direction"),
                None => panic!("expected direction"),
            };
            let number_of_times = splitted_line.next().map(|v|v.parse::<u32>().unwrap()).unwrap();
            for _ in 0..number_of_times {
                head_knot = move_function(head_knot);
                if !is_knot_adjacent(&head_knot, &knots[0]) {
                    knots[0] = move_knot(knots[0], &head_knot);
                    for i in 1..9 {
                        if !is_knot_adjacent(&knots[i], &knots[i-1]) {
                            knots[i] = move_knot(knots[i], &knots[i-1]);
                        }
                    }
                    if !tiles_tail_visited.contains(&knots[8]) {
                        tiles_tail_visited.push(knots[8]);
                    }
                }
            }
        }
    }
    Ok(tiles_tail_visited.len() as u32)
}
