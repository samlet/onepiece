/*
⊕ [std :: sync :: Arc - Rust](https://doc.rust-lang.org/std/sync/struct.Arc.html)

线程安全的引用计数指针。'Arc'代表'Atomically Reference Counted'。
该类型Arc<T>提供T了在堆中分配的类型值的共享所有权。调用clone上Arc产生一个新的Arc实例，它指向在堆上作为源相同的值Arc，同时增加一个引用计数。当Arc 销毁给定值的最后一个指针时，指向的值也会被销毁。
Rust中的共享引用默认情况下禁止突变，Arc也不例外：通常不能获取对内部内容的可变引用Arc。
如果你需要通过一个发生变异Arc，使用 Mutex，RwLock或者一个Atomic 类型。

与之不同Rc<T>，Arc<T>使用原子操作进行引用计数。这意味着它是线程安全的。
缺点是原子操作比普通的存储器访问更昂贵。如果您不在线程之间共享引用计数值，
请考虑使用 Rc<T>较低的开销。Rc<T>是一个安全的默认值，因为编译器将捕获任何尝试Rc<T>在线程之间发送。
但是，图书馆可能会选择Arc<T>以便为图书馆消费者提供更大的灵活性。
Arc<T>将实现Send和Sync只要T工具 Send和Sync。
为什么不能在非线程安全类型T中 Arc<T>使其成为线程安全的？起初这可能有点违反直觉：
毕竟，这不是Arc<T>线程安全的重点吗？关键是：Arc<T>使线程安全，拥有相同数据的多个所有权，
但它不会为其数据添加线程安全性。考虑 。不是，如果一直如此 ，也会如此。
但后来我们遇到了一个问题： 不是线程安全的; 它使用非原子操作跟踪借用计数。
*/

fn clone_cl(){
    // Creating a new reference from an existing reference counted pointer
    // is done using the Clone trait implemented for Arc<T> and Weak<T>.
    use std::sync::Arc;
    let foo = Arc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent.
    let a = foo.clone();
    let b = Arc::clone(&foo);
    // a, b, and foo are all Arcs that point to the same memory location
}

fn share_immutable_cl(){
    use std::sync::Arc;
    use std::thread;

    let five = Arc::new(5);

    for _ in 0..10 {
        let five = Arc::clone(&five);

        thread::spawn(move || {
            println!("{:?}", five);
        });
    }
}

fn share_mutable_cl(){
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    let val = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let val = Arc::clone(&val);

        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
        });
    }
}

fn main() {
    clone_cl();
    share_immutable_cl();
    share_mutable_cl();
}