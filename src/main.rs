mod day1;

fn timed(f: fn()) {
    let start = std::time::Instant::now();
    f();
    println!("\tCompleted in {:?}", std::time::Instant::now() - start);
}
fn main() {
    day1::solve();
}
