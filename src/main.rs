#![feature(portable_simd)]
mod day1;
mod day2;
fn timed<T: Fn()>(f: fn() -> T, trials: u32) {
    assert!(trials >= 1);
    let start = std::time::Instant::now();
    let result = f();
    for _ in 1..trials {
        f();
    }
    let time = std::time::Instant::now() - start;
    result();
    if trials > 1 {
        println!("\tCompleted {trials} trials in {:?}, average of {:?}", time, time/trials);
    } else {
        println!("\tCompleted in {:?}", time);
    }
}
fn main() {
    day1::solve();
    day2::solve();
}
