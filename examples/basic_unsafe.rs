//⊕ [不安全 Rust | Rust 中文](https://rustlang-cn.org/office/rust/book/advanced-features/ch19-01-unsafe-rust.html)
extern "C" {
    fn abs(input: i32) -> i32;
}

fn c_cl() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 读取或修改一个可变静态变量是不安全的
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 拥有可以全局访问的可变数据，难以保证不存在数据竞争，这就是为何 Rust 认为可变静态变量是不安全的。
// 优先使用并发技术和线程安全智能指针
fn static_var_cl() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    c_cl();
    static_var_cl();
}