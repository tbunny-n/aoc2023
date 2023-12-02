#![allow(dead_code, unused_imports)]
#![allow(warnings)]
use std::collections::HashMap;

use crate::problems::get_input;

const P1_TEST: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

pub fn run() {
    let input = get_input("p1");
    // let input = P1_TEST;

    // Split input by linebreaks
    let lines = input.lines();

    let mut numbers: Vec<i32> = Vec::new();

    // O(lines)
    for line in lines {
        let chars = line.chars();

        let mut first_num_char: char = ' ';
        let mut last_num_char: char = ' ';

        // O(chars)
        for c in chars {
            if c.is_ascii_digit() {
                if first_num_char == ' ' {
                    first_num_char = c;
                } else {
                    last_num_char = c;
                }
            }
        }

        // Number requires 2 digits
        if last_num_char == ' ' {
            last_num_char = first_num_char;
        }

        // Combine chars
        let mut combined_num_string = String::new();
        combined_num_string.push(first_num_char);
        combined_num_string.push(last_num_char);

        // Parse to i32
        let parsed_num: i32 = combined_num_string.parse().unwrap_or(0);
        numbers.push(parsed_num);
    }

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}

const REPLACEMENT_PAIRS: [(&'static str, &'static str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

pub fn run2() {
    let input = get_input("p1");
    let parsed_input = parse_word_nums(input);

    fn parse_word_nums(whole_input: &str) -> String {
        let mut whole_input = whole_input.to_owned();
        for pair in REPLACEMENT_PAIRS {
            whole_input = whole_input.replace(pair.0, pair.1);
        }
        whole_input
    }

    let mut numbers: Vec<i32> = Vec::new();

    for line in parsed_input.lines() {
        let mut num_string = String::new();
        let mut first_num_char = ' ';
        let mut last_num_char = ' ';
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first_num_char == ' ' {
                    first_num_char = c;
                } else {
                    last_num_char = c;
                }
            }
        }
        if last_num_char == ' ' {
            last_num_char = first_num_char;
        }

        num_string.push(first_num_char);
        num_string.push(last_num_char);

        let full_num = num_string.parse::<i32>().unwrap_or(0);
        numbers.push(full_num);
    }

    println!("Numbers: {:?}", numbers);

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {sum}");
}
