use std::time::{Duration, Instant};
use std::thread::sleep;

fn elapsed_cl() {
    let now = Instant::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 0));
    // it prints '2'
    println!("{}", now.elapsed().as_secs());
}

//⊕ [定时器 | Rust 中文](https://rustlang-cn.org/crates/tokio/docs/going-deeper/timers.html)
use tokio::prelude::*;
use tokio::timer::Delay;

// 与所有future一样，延迟是懒惰的。 简单地创建一个新的Delay实例什么都不做。
// 该实例必须用于生成到Tokio运行时的任务。 运行时预先配置了一个计时器实现，
// 以驱动Delay实例完成。 示例中，这是通过将任务传递给tokio :: run来完成的。
// 使用tokio :: spawn也可以。
fn task_cl() {
    let when = Instant::now() + Duration::from_millis(100);
    let task = Delay::new(when)
        .and_then(|_| {
            println!("Hello world!");
            Ok(())
        })
        .map_err(|e| panic!("delay errored; err={:?}", e));

    tokio::run(task);
}

use tokio::timer::Interval;
fn interval_cl() {
    let task = Interval::new(Instant::now(), Duration::from_millis(100))
        .take(10)
        .for_each(|instant| {
            println!("fire; instant={:?}", instant);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}

fn main() {
    task_cl();
    // elapsed_cl();
    interval_cl();
}

