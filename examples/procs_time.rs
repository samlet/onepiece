use std::time::{Duration, Instant};
use std::thread::sleep;

//⊕ [std::time::Instant - Rust](https://doc.rust-lang.org/std/time/struct.Instant.html)

fn elapsed_cl() {
    let now = Instant::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 0));
    // it prints '2'
    println!("{}", now.elapsed().as_secs());
}

fn main() {
    elapsed_cl();
}

