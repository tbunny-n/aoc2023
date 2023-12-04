use std::collections::HashSet;

use super::get_input;

const TEST_INPUT: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn run() {
    let input = get_input("d4");
    // let input = TEST_INPUT;

    let mut set1: HashSet<i32>;
    let mut set2: HashSet<i32>;

    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        // strip prefix
        let line = line.split(':').nth(1).unwrap().trim();

        let mut winning_current = line.split('|');
        let winning = winning_current.next().unwrap().trim();
        let current = winning_current.next().unwrap().trim();

        let mut points = 0;

        // For every number in current that is in winning, double the points
        set1 = current
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        set2 = winning
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        set1.intersection(&set2)
            .for_each(|_| if points == 0 { points = 1 } else { points *= 2 });

        sum += points;
    }

    println!("Sum: {}", sum);
}
