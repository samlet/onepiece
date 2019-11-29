//⊕ [创建任务 | Rust 中文](https://rustlang-cn.org/crates/tokio/docs/futures/spawning.html)
/*
基于 Tokio 的应用程序是以任务（task）为单位组织的。任务是较小的独立运行的逻辑单元。
类似于 Go 语言的 goroutine 和 Erlang 的 process。
换句话说，任务是异步的绿色线程（green thread）。
创建（spawn）任务与使用同步代码创建线程往往出于相似的原因，但是使用 Tokio 创建任务非常轻量。

之前的一些例子定义 future 并将其传递给 tokio::run 函数。
这样就会在 Tokio 的运行时上创建一个任务并执行。
更多的任务可能需要通过调用 tokio::spawn 来创建，这仅限于那些已经作为 Tokio 任务运行的代码。
有一个帮助理解的好方法，我们可以把传递给 tokio::run 函数的 future 视为 “main 函数”。
*/
extern crate tokio;
extern crate futures;

//use futures::{stream, Future, Stream, Sink};
use futures::future::lazy;
//use futures::sync::mpsc;

// 只能在futures="0.1.*"下使用, 见bigmess/procs_tokio_task.rs
fn basic_cl() {
    /*
    tokio::run(lazy(|| {
        let (tx, rx) = mpsc::channel(1_024);

        tokio::spawn({
            stream::iter_ok(0..10).fold(tx, |tx, i| {
                tx.send(format!("Message {} from spawned task", i))
                    .map_err(|e| println!("error = {:?}", e))
            })
                .map(|_| ()) // 释放 tx 句柄
        });

        rx.for_each(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
    }));
    */
}

fn main() {
    basic_cl();
}