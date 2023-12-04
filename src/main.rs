use std::time::Instant;

mod problems;

fn main() {
    let before = Instant::now();
    problems::d3::run2();
    println!("Time: {}ms", before.elapsed().as_millis());
}
