//⊕ [共享状态并发 | Rust 中文](https://rustlang-cn.org/office/rust/book/concurrency/ch16-03-shared-state.html)
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::thread;

// Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。
// 这个智能指针实现了 Deref 来指向其内部数据；
// 其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
fn basic_cl() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Rc<T> 并不能安全的在线程间共享。当 Rc<T> 管理引用计数时，
// 它必须在每一个 clone 调用时增加计数，并在每一个克隆被丢弃时减少计数。
// Rc<T> 并没有使用任何并发原语，来确保改变计数的操作不会被其他线程打断。
// 在计数出错时可能会导致诡异的 bug，比如可能会造成内存泄漏，
// 或在使用结束之前就丢弃一个值。我们所需要的是一个完全类似 Rc<T>，
// 又以一种线程安全的方式改变引用计数的类型。

// Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。
// 字母 “a” 代表 原子性（atomic），所以这是一个原子引用计数（atomically reference counted）类型。
// 原子性是另一类这里还未涉及到的并发原语：请查看标准库中 std::sync::atomic 的文档来获取更多细节。
// 其中的要点就是：原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。

fn arc_cl() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    basic_cl();
    arc_cl();
}