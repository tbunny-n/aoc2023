use super::get_input;
use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
const GEAR_SYMBOL: char = '*';

fn is_symbol(c: char) -> bool {
    c != '.' && c != ' ' && c != '\n' && !c.is_ascii_digit()
}

pub fn run() {
    let input = get_input("d3");
    // let input = TEST_INPUT;

    // (col, row)
    let mut symbol_coords = HashSet::new();

    // Aggregate symbols
    for (col, input) in input.lines().enumerate() {
        for (row, c) in input.chars().enumerate() {
            if is_symbol(c) {
                symbol_coords.insert((col, row));
            }
        }
    }

    let mut sum: i32 = 0;

    // Get numbers adjacent to symbols
    for (col, line) in input.lines().enumerate() {
        let mut current_number = String::new();
        let mut is_adjacent = false;
        for (row, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                // Build number
                current_number.push(c);

                // Only check on numbers that have no digits with adjacent symbols yet
                if !is_adjacent {
                    // Check adjacency
                    let adjacent_coords = [
                        // Vertical
                        (col.saturating_sub(1), row),
                        (col.saturating_add(1), row),
                        // Horizontal
                        (col, row.saturating_sub(1)),
                        (col, row.saturating_add(1)),
                        // Diagonal
                        (col.saturating_sub(1), row.saturating_sub(1)),
                        (col.saturating_add(1), row.saturating_add(1)),
                        (col.saturating_sub(1), row.saturating_add(1)),
                        (col.saturating_add(1), row.saturating_sub(1)),
                    ];

                    // Check if any of the adjacent coords are in the symbol coords
                    for coord in adjacent_coords.iter() {
                        if symbol_coords.contains(coord) {
                            is_adjacent = true;
                            break;
                        }
                    }
                }

                // If next char is not a digit i.e. end of number or end of line,
                // add number (if adjacent) and reset vars
                if let Some(next_char) = line.chars().nth(row + 1) {
                    // Next char is not a digit
                    if !next_char.is_ascii_digit() {
                        if is_adjacent {
                            // println!("Found number: {}", current_number);
                            sum += current_number.parse::<i32>().unwrap();
                        }
                        current_number.clear();
                        is_adjacent = false;
                    }
                }
                // End of line
                else {
                    if is_adjacent {
                        // println!("Found number: {}", current_number);
                        sum += current_number.parse::<i32>().unwrap();
                    }
                    current_number.clear();
                    is_adjacent = false;
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

pub fn run2() {
    let input = get_input("d3");

    // (col, row) = [number, number]
    // I'm going to store adjacent numbers into the gear's coordinate key.
    // If it already has two numbers, I can skip.. I think.
    let mut gear_map: HashMap<(usize, usize), [i32; 2]> = HashMap::new();

    let mut sum: i64 = 0;

    // Aggregate gears
    for (col, line) in input.lines().enumerate() {
        for (row, c) in line.chars().enumerate() {
            if c == GEAR_SYMBOL {
                gear_map.insert((col, row), [0, 0]);
            }
        }
    }

    // Get numbers adjacent to symbols
    for (col, line) in input.lines().enumerate() {
        let mut current_number = String::new();
        let mut is_adjacent = false;
        let mut last_gear_coord: Option<(usize, usize)> = None;

        for (row, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                // Build number
                current_number.push(c);

                // Only check on numbers that have no digits with adjacent gears yet
                if !is_adjacent {
                    // Check adjacency
                    let adjacent_coords = [
                        // Vertical
                        (col.saturating_sub(1), row),
                        (col.saturating_add(1), row),
                        // Horizontal
                        (col, row.saturating_sub(1)),
                        (col, row.saturating_add(1)),
                        // Diagonal
                        (col.saturating_sub(1), row.saturating_sub(1)),
                        (col.saturating_add(1), row.saturating_add(1)),
                        (col.saturating_sub(1), row.saturating_add(1)),
                        (col.saturating_add(1), row.saturating_sub(1)),
                    ];

                    // Check if any of the adjacent coords are in the gear coords
                    for coord in adjacent_coords.iter() {
                        if gear_map.contains_key(coord) {
                            is_adjacent = true;
                            last_gear_coord = Some(*coord);
                            break;
                        }
                    }
                }

                // If next char is not a digit i.e. end of number or end of line,
                // add number to gear coordinate (if adjacent) and reset vars
                if let Some(next_char) = line.chars().nth(row + 1) {
                    // Next char is not a digit
                    if !next_char.is_ascii_digit() {
                        if is_adjacent && last_gear_coord.is_some() {
                            // println!("Found number: {}", current_number);
                            let gear_coord = last_gear_coord.unwrap();
                            let val = gear_map.get_mut(&gear_coord).unwrap();
                            if val[0] == 0 {
                                val[0] = current_number.parse::<i32>().unwrap();
                            } else {
                                val[1] = current_number.parse::<i32>().unwrap();
                            }
                            println!("{:?}", gear_map.get(&gear_coord));
                        }
                        current_number.clear();
                        last_gear_coord = None;
                        is_adjacent = false;
                    }
                }
                // End of line
                else {
                    if is_adjacent && last_gear_coord.is_some() {
                        // println!("Found number: {}", current_number);
                        let gear_coord = last_gear_coord.unwrap();
                        let val = gear_map.get_mut(&gear_coord).unwrap();
                        if val[0] == 0 {
                            val[0] = current_number.parse::<i32>().unwrap();
                        } else {
                            val[1] = current_number.parse::<i32>().unwrap();
                        }
                        println!("{:?}", gear_map.get(&gear_coord));
                    }
                    current_number.clear();
                    last_gear_coord = None;
                    is_adjacent = false;
                }
            }
        }
    }

    // Get all gears with two numbers that aren't zero and multiply them together
    for numbers in gear_map.values() {
        if numbers[0] != 0 && numbers[1] != 0 {
            sum += numbers[0] as i64 * numbers[1] as i64;
        }
    }

    println!("Sum: {}", sum);
}
