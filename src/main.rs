use std::time::Instant;

mod problems;

fn main() {
    let before = Instant::now();
    problems::p2::run2();
    println!("Time: {}ms", before.elapsed().as_millis());
}
