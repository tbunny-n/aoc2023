#![allow(dead_code)]

pub mod d1;
pub mod d2;
pub mod d3;

pub fn get_input(day: &str) -> String {
    let file_dir = format!("inputs/{}.txt", day);
    std::fs::read_to_string(file_dir).unwrap()
}
