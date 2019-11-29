use tokio_timer::*;

//use futures::Stream;
use std::time::Duration;
use tokio::timer::DelayQueue;
use std::thread;

//⊕ [tokio_timer::DelayQueue - Rust](https://docs.rs/tokio-timer/0.2.6/tokio_timer/struct.DelayQueue.html)

fn main() {
    let delay_queue: DelayQueue<u32> = DelayQueue::new();
    let mut delay_queue = DelayQueue::with_capacity(10);

    // let now = time.now();
    // These insertions are done without further allocation
    for i in 0..10 {
        // delay_queue.insert(i, Duration::from_secs(i));
        delay_queue.insert(i, Duration::from_millis(i));
    }

    // This will make the queue allocate additional storage
    // delay_queue.insert(11, Duration::from_secs(11));
    delay_queue.insert(11, Duration::from_millis(11));

    println!("is empty: {}", delay_queue.is_empty());

    println!("wait 5 secs ..");
    // thread::sleep(Duration::from_millis(5000));
    thread::sleep(Duration::from_millis(5));
    println!("is empty: {}, capacity: {}", delay_queue.is_empty(), delay_queue.capacity());

    println!("wait 7 secs ..");
    // thread::sleep(Duration::from_millis(6000));
    thread::sleep(Duration::from_millis(10));
    println!("is empty: {}, capacity: {}", delay_queue.is_empty(), delay_queue.capacity());

    // ?
}
