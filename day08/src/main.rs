use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match part01() {
        Ok(number_of_visible_trees) => println!(
            "number of visible trees part1: {}",
            number_of_visible_trees
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(highest_scenic_score) => println!(
            "highest scenic score part2: {}",
            highest_scenic_score
        ),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

fn is_visible(i: usize, j: usize, tree_columns: &Vec<Vec<u32>>) -> bool {
    let current_val = tree_columns[i][j];

    // left
    if tree_columns[i].iter().take(j).all(|t| t < &current_val) {
        return true;
    }
    // right
    if tree_columns[i].iter().skip(j+1).all(|t| t < &current_val) {
        return true;
    }

    //up
    let mut visible = true;
    for up in 0..i {
        if tree_columns[up][j] >= current_val {
            visible = false;
            break;
        }
    }
    if visible {
        return visible;
    }

    // down
    visible = true;
    for down in (i+1)..tree_columns.len() {
        if tree_columns[down][j] >= current_val {
            visible = false;
            break;
        }
    }
    visible
}

fn part01() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut number_of_visible_trees = 0;
    let mut tree_columns = vec![];
    let mut number_of_trees_per_row:usize = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let tree_row:Vec<u32> = line_content.chars().map(|c| c.to_digit(10).unwrap()).collect();
            if number_of_trees_per_row == 0 {
                number_of_trees_per_row = tree_row.len();
            }
            tree_columns.push(tree_row);
        }
    }

    for i in 1..(tree_columns.len() - 1) {
        for j in 1..(number_of_trees_per_row - 1) {
            if is_visible(i, j, &tree_columns) {
                number_of_visible_trees += 1;
            }
        }
    }
    let number_of_outer_trees = ((tree_columns.len() * 2) + (number_of_trees_per_row * 2) - 4) as u32;
    number_of_visible_trees += number_of_outer_trees;
    Ok(number_of_visible_trees)
}

fn get_scenic_score(i: usize, j: usize, tree_columns: &Vec<Vec<u32>>) -> u32 {
    let current_val = tree_columns[i][j];
    // left
    let mut scenic_score_left:u32 = 0;
    for left in (0..j).rev() {
        scenic_score_left += 1;
        if tree_columns[i][left] >= current_val {
            break;
        }
    }
    
    // right
    // done like this to not have to know how long a line is for the for loop
    let mut encountered_blocking_tree = false;
    let scenic_score_right = tree_columns[i].iter()
    .skip(j+1)
    .take_while(|t| {
        if encountered_blocking_tree {
            return false;
        }
        if t >= &&current_val {
            encountered_blocking_tree = true;
            return true;
        }
        t < &&current_val
    })
    .count() as u32;

    //up
    let mut scenic_score_up:u32 = 0;
    for up in (0..i).rev() {
        scenic_score_up += 1;
        if tree_columns[up][j] >= current_val {
            break;
        }
    }

    // down
    let mut scenic_score_down:u32 = 0;
    for down in (i+1)..tree_columns.len() {
        scenic_score_down += 1;
        if tree_columns[down][j] >= current_val {
            break;
        }
    }
    scenic_score_left * scenic_score_right * scenic_score_up * scenic_score_down
}

fn part02() -> io::Result<u32> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut tree_columns = vec![];
    let mut number_of_trees_per_row:usize = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_content) = line {
            let tree_row:Vec<u32> = line_content.chars().map(|c| c.to_digit(10).unwrap()).collect();
            if number_of_trees_per_row == 0 {
                number_of_trees_per_row = tree_row.len();
            }
            tree_columns.push(tree_row);
        }
    }
    let mut current_best_scenic_score:u32 = 0;
    for i in 1..(tree_columns.len() - 1) {
        for j in 1..(number_of_trees_per_row - 1) {
            let current_scenic_score = get_scenic_score(i, j, &tree_columns);
            if current_scenic_score > current_best_scenic_score {
                current_best_scenic_score = current_scenic_score;
            }
        }
    }
    Ok(current_best_scenic_score)
}
