#![allow(dead_code, unused_imports)]
use super::get_input;
use std::collections::HashSet;

const TEST_INPUT: &str = "
.%....
..#.1.
.@.#..
....34";

fn is_symbol(c: char) -> bool {
    c != '.' && c != ' ' && c != '\n' && !c.is_ascii_digit()
}

pub fn run() {
    let input = get_input("p3");
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

                // Only check on numbers that have no adjecent digits yet
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
