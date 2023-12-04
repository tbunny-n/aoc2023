use std::time::Instant;

mod problems;

fn main() {
    let before = Instant::now();
    problems::p3::run();
    println!("Time: {}ms", before.elapsed().as_millis());
}
