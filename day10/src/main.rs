use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(sum_of_signal_strengths) => println!(
            "sum of signal strengths part1: {}",
            sum_of_signal_strengths
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(drawn_sprite) => println!(
            "drawn sprite part2: {}",
            drawn_sprite
        ),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn part01() -> io::Result<i32> {
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
            let mut splitted_line = line_content.split_whitespace();
            let command_result = match splitted_line.next() {
                Some("noop") => (1, 0),
                Some("addx") => (2, splitted_line.next().map(|v| v.parse::<i32>().unwrap()).unwrap()),
                _ => panic!("expected noop or addx with one argument"),
            };
            current_cycle += command_result.0;
            if current_cycle >= target_cycle && target_cycle < 221 {
                signal_strength += target_cycle * current_value;
                target_cycle += 40;
            }
            current_value += command_result.1;
        }
    }
    Ok(signal_strength)
}


fn part02() -> io::Result<String> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut screen_output:Vec<String> = vec!["".to_string(); 6];
    let mut current_cycle = 0;
    let mut sprite_position:i32 = 1;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let mut splitted_line = line_content.split_whitespace();
            let command_result = match splitted_line.next() {
                Some("noop") => (1, 0),
                Some("addx") => (2, splitted_line.next().map(|v| v.parse::<i32>().unwrap()).unwrap()),
                _ => panic!("expected noop or addx with one argument"),
            };
            for i in 0..command_result.0 {
                let next_cycle = current_cycle + i;
                let screen_row = next_cycle / 40;
                let current_pixel = next_cycle % 40;
                if (sprite_position - 1..=sprite_position + 1).contains(&current_pixel) {
                    screen_output[screen_row as usize].push('#')
                } else {
                    screen_output[screen_row as usize].push('.')
                }
            }
            current_cycle += command_result.0;
            sprite_position += command_result.1;
        }
    }
    let drawn_sprite = screen_output.into_iter()
    .fold("".to_string(), |a, b| a + "\n" + &b);
    Ok(drawn_sprite)
}
