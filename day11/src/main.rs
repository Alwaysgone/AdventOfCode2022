use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

fn main() {
    match part01() {
        Ok(level_of_monkey_business) => println!(
            "level of monkey business after 20 rounds part1: {}",
            level_of_monkey_business
        ),
        Err(e) => println!("an error occurred in part01: {}", e),
    }
    match part02() {
        Ok(level_of_monkey_business) => println!(
            "level of monkey business after 10000 rounds part2: {}",
            level_of_monkey_business
        ),
        Err(e) => println!("an error occurred in part02: {}", e),
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items:Vec<u64>,
    operation:fn(u64) -> u64,
    test:u64,
    next_monkey_if_true:u32,
    next_monkey_if_false:u32,
    inspected_items:u64,
}

type Monkeys = HashMap<u32, Monkey>;


fn part01() -> io::Result<u64> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = io::BufReader::new(file).lines();
    let mut monkeys = Monkeys::new();

    while let Some((i, monkey)) = parse_monkey(&mut lines) {
        monkeys.insert(i as u32, monkey);
    }

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            monkeys = process_monkey(monkeys, monkey_index as u32, Box::new(|x| x / 3));
        }
    }

    let mut inspected_items:Vec<u64> = monkeys.values().map(|m| m.inspected_items)
    .collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    let monkey_business = match inspected_items.into_iter()
    .take(2)
    .reduce(|a, b| a * b) {
        Some(monkey_business) => monkey_business,
        _ => panic!("could not calculate monkey business"),
    };
    
    Ok(monkey_business)
}

fn parse_monkey(input: &mut Lines<BufReader<File>>) -> Option<(u64, Monkey)> {
    let monkey_header_text = input.next();
    if let Some(Ok(monkey_header)) = monkey_header_text {
        let monkey_index = monkey_header[7..8].parse::<u64>().unwrap();
        let items_text = input.next().unwrap().unwrap().trim().to_owned();
        let items:Vec<u64> = (&items_text[16..items_text.len()])
        .split(", ")
        .map(|i| i.parse::<u64>().unwrap())
        .collect();
        let operation_text = input.next().unwrap().unwrap().trim().to_owned();
        let operation:fn(u64) -> u64 = match &operation_text[17..operation_text.len()] {
            "old * 19" => |x| x * 19,
            "old + 6" => |x| x + 6,
            "old * old" => |x| x * x,
            "old + 3" => |x| x + 3,
            "old * 2" => |x| x * 2,
            "old + 2" => |x| x + 2,
            "old * 11" => |x| x * 11,
            "old + 7" => |x| x + 7,
            "old + 1" => |x| x + 1,
            "old + 5" => |x| x + 5,
            _ => panic!("unexpected symbol for monkey operation"),
        };
        let test_text = input.next().unwrap().unwrap().trim().to_owned();
        let test = test_text[19..test_text.len()].parse::<u64>().unwrap();
        let next_monkey_if_true_text = input.next().unwrap().unwrap().trim().to_owned();
        let next_monkey_if_true = next_monkey_if_true_text[25..next_monkey_if_true_text.len()].parse::<u32>().unwrap();
        let next_monkey_if_false_text = input.next().unwrap().unwrap().trim().to_owned();
        let next_monkey_if_false = next_monkey_if_false_text[26..next_monkey_if_false_text.len()].parse::<u32>().unwrap();
        input.next();
        Some((monkey_index, Monkey {
            items: items,
            operation: operation,
            test: test,
            next_monkey_if_true: next_monkey_if_true,
            next_monkey_if_false: next_monkey_if_false,
            inspected_items: 0
        }))
    } else {
        None
    }
}

fn process_monkey(monkeys:Monkeys, monkey_index:u32, worry_reduction:Box<dyn Fn(u64) -> u64 + '_>) -> Monkeys {
    let monkey = monkeys.get(&monkey_index).unwrap();
    let mut new_monkeys = monkeys.clone();
    for item in monkey.items.iter() {
        let operation = monkey.operation;
        let new_item = worry_reduction(operation(*item));
        let other_monkey_index;
        if new_item % monkey.test == 0 {
            other_monkey_index = monkey.next_monkey_if_true;
        } else {
            other_monkey_index = monkey.next_monkey_if_false;
        }
        new_monkeys.get_mut(&other_monkey_index).unwrap().items.push(new_item);
    }
    let updated_monkey = new_monkeys.get_mut(&monkey_index).unwrap();
    updated_monkey.inspected_items += updated_monkey.items.len() as u64;
    updated_monkey.items.clear();
    new_monkeys
}

fn part02() -> io::Result<u64> {
    let path = Path::new("./input/input.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("could not open input file: {}", e),
    };
    let mut lines = io::BufReader::new(file).lines();
    let mut monkeys = Monkeys::new();

    while let Some((i, monkey)) = parse_monkey(&mut lines) {
        monkeys.insert(i as u32, monkey);
    }
    let test_product= monkeys.values().map(|m| m.test)
    .reduce(|a, b| a * b).unwrap().to_owned();
    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            monkeys = process_monkey(monkeys, monkey_index as u32, Box::new(|item:u64| -> u64 {
                item % test_product
            }));
        }
    }

    let mut inspected_items:Vec<u64> = monkeys.values().map(|m| m.inspected_items)
    .collect();
    inspected_items.sort_by(|a, b| b.cmp(a));
    let monkey_business = match inspected_items.into_iter()
    .take(2)
    .reduce(|a, b| a * b) {
        Some(monkey_business) => monkey_business,
        _ => panic!("could not calculate monkey business"),
    };
    
    Ok(monkey_business)
}
