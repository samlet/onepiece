extern crate crossbeam_deque as deque;
extern crate crossbeam_utils as utils;
extern crate rand;

use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Mutex};

use deque::Steal::{Empty, Success};
use deque::Worker;
use rand::Rng;
use utils::thread::scope;

//⊕ [crossbeam::deque - Rust](https://docs.rs/crossbeam/*/crossbeam/deque/index.html)
//⊕ [crossbeam/crossbeam-deque at master · crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-deque)
//⊕ [crossbeam/crossbeam-utils at master · crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-utils)

fn smoke() {
    let w = Worker::new_lifo();
    let s = w.stealer();
    assert_eq!(w.pop(), None);
    assert_eq!(s.steal(), Empty);

    w.push(1);
    assert_eq!(w.pop(), Some(1));
    assert_eq!(w.pop(), None);
    assert_eq!(s.steal(), Empty);

    w.push(2);
    assert_eq!(s.steal(), Success(2));
    assert_eq!(s.steal(), Empty);
    assert_eq!(w.pop(), None);

    w.push(3);
    w.push(4);
    w.push(5);
    assert_eq!(s.steal(), Success(3));
    assert_eq!(s.steal(), Success(4));
    assert_eq!(s.steal(), Success(5));
    assert_eq!(s.steal(), Empty);

    w.push(6);
    w.push(7);
    w.push(8);
    w.push(9);
    assert_eq!(w.pop(), Some(9));
    assert_eq!(s.steal(), Success(6));
    assert_eq!(w.pop(), Some(8));
    assert_eq!(w.pop(), Some(7));
    assert_eq!(w.pop(), None);
}

fn spsc() {
    const STEPS: usize = 50_000;

    let w = Worker::new_lifo();
    let s = w.stealer();

    scope(|scope| {
        scope.spawn(|_| {
            for i in 0..STEPS {
                loop {
                    if let Success(v) = s.steal() {
                        assert_eq!(i, v);
                        break;
                    }
                }
            }

            assert_eq!(s.steal(), Empty);
        });

        for i in 0..STEPS {
            w.push(i);
        }
    })
        .unwrap();
}

fn main() {
    smoke();
    spsc();
}