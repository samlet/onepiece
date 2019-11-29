//⊕ [通道 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/std_misc/channels.html)
//Rust 为线程之间的通信提供了异步的通道（channel）。通道允许两个端点之间信息的 单向流动：Sender（发送端） 和 Receiver（接收端）。
//⊕ [使用消息传递在线程间传送数据 | Rust 中文](https://rustlang-cn.org/office/rust/book/concurrency/ch16-02-message-passing.html)

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

static NTHREADS: i32 = 3;

fn basic_cl() {
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送
    // 的消息的类型（类型标注是可选的）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // sender 端可被复制
        let thread_tx = tx.clone();

        // 每个线程都将通过通道来发送它的 id
        thread::spawn(move || {
            // 被创建的线程取得 `thread_tx` 的所有权
            // 每个线程都把消息放在通道的消息队列中
            thread_tx.send(id).unwrap();

            // 发送是一个非阻塞（non-blocking）操作，线程将在发送完消息后
            // 会立即继续进行
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv` 方法从通道中拿到一个消息
        // 若无可用消息的话，`recv` 将阻止当前线程
        ids.push(rx.recv());
    }

    // 显示消息被发送的次序
    println!("{:?}", ids);
}

/*
通道的接收端有两个有用的方法：recv 和 try_recv。这里，我们使用了 recv，
它是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值。
一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。
当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。

try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，
而 Err 值代表此时没有任何消息。如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：
可以编写一个循环来频繁调用 try_recv，再有可用消息时进行处理，
其余时候则处理一会其他工作直到再次检查。
*/
fn string_cl() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// 遍历他们，单独的发送每一个字符串并通过一个 Duration 值调用 thread::sleep 函数来暂停一秒。
// 不再显式调用 recv 函数：而是将 rx 当作一个迭代器。对于每一个接收到的值，
// 我们将其打印出来。当通道被关闭时，迭代器也将结束。
fn iter_cl() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//通过克隆发送者来创建多个生产者
fn producer_cl(){
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    basic_cl();
    string_cl();
    iter_cl();
    producer_cl();
}