#![feature(portable_simd)]
mod day1;
mod day2;
fn timed(f: fn()) {
    let start = std::time::Instant::now();
    f();
    println!("\tCompleted in {:?}", std::time::Instant::now() - start);
}
fn main() {
    day1::solve();
    day2::solve();
}
