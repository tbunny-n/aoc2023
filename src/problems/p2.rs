#![allow(dead_code, unused_imports)]
use super::get_input;

const RED_CUBE_COUNT: i32 = 12;
const GREEN_CUBE_COUNT: i32 = 13;
const BLUE_CUBE_COUNT: i32 = 14;

pub fn run() {
    //     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let input = get_input("p2");

    let mut sum = 0;
    for (game_num, line) in input.lines().enumerate() {
        // Strip prefix
        let line = line.split(':').nth(1).unwrap().trim();

        // The number is always followed by the corresponding color
        // I think we can loop through each word, check if it's a number, and then check the next word
        // The strings currently look like `3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`

        let line = line.replace(",", "");
        let sets = line.split(';');

        let mut is_set_ruined = false;
        for set in sets {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            let words = set.split(' ').collect::<Vec<&str>>();

            for i in 0..words.len() {
                if let Ok(num) = words[i].parse::<i32>() {
                    match words[i + 1] {
                        "red" => red_count += num,
                        "green" => green_count += num,
                        "blue" => blue_count += num,
                        _ => (),
                    }
                }
            }

            if red_count > RED_CUBE_COUNT
                || green_count > GREEN_CUBE_COUNT
                || blue_count > BLUE_CUBE_COUNT
            {
                is_set_ruined = true;
            }
        }

        if !is_set_ruined {
            println!("Won game {}", game_num + 1);
            sum += game_num + 1;
        } else {
            println!("-- ! Lost game {}", game_num + 1);
        }
    }

    println!("Sum: {}", sum);
}

pub fn run2() {
    let input = get_input("p2");

    let mut sum = 0;

    for line in input.lines() {
        // Strip prefix
        let line = line.split(':').nth(1).unwrap().trim();

        // Remove punctuation
        let line = line.replace(",", "");
        let line = line.replace(";", "");

        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;

        let words = line.split_whitespace().collect::<Vec<&str>>();
        for i in 0..words.len() {
            if let Ok(num) = words[i].parse::<i32>() {
                let color = words[i + 1];
                match color {
                    "red" => {
                        if num > highest_red {
                            highest_red = num;
                        }
                    }
                    "green" => {
                        if num > highest_green {
                            highest_green = num;
                        }
                    }
                    "blue" => {
                        if num > highest_blue {
                            highest_blue = num;
                        }
                    }
                    _ => (),
                }
            }
        }

        let power = highest_red * highest_green * highest_blue;
        sum += power;
    }

    println!("Sum: {}", sum);
}
