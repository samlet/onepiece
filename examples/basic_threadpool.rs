//⊕ [threadpool - Rust](https://docs.rs/threadpool/1.7.1/threadpool/)
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn sync_with_channel() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

fn sync_with_barrier(){
    use std::sync::{Arc, Barrier};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // create at least as many workers as jobs or you will deadlock yourself
    let n_workers = 42;
    let n_jobs = 23;
    let pool = ThreadPool::new(n_workers);
    let an_atomic = Arc::new(AtomicUsize::new(0));

    assert!(n_jobs <= n_workers, "too many jobs, will deadlock");

    // create a barrier that waits for all jobs plus the starter thread
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    for _ in 0..n_jobs {
    let barrier = barrier.clone();
    let an_atomic = an_atomic.clone();

    pool.execute(move|| {
    // do the heavy work
    an_atomic.fetch_add(1, Ordering::Relaxed);

    // then wait for the other threads
    barrier.wait();
    });
    }

    // wait for the threads to finish the work
    barrier.wait();
    assert_eq!(an_atomic.load(Ordering::SeqCst), 23);
}

fn main() {
    sync_with_channel();
    sync_with_barrier();
}