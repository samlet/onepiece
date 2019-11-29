//⊕ [类型匿名 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/anonymity.html)
// 闭包从周围的作用域中捕获变量是简单明了的。这样会有某些后果吗？确实有。
// 观察一下 使用闭包作为函数参数，这要求闭包是泛型的，闭包定义的方式决定了这是 必要的。
// 若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的。
// 然 而，使用未限定类型的参数 <T> 过于不明确，并且是不允许的。
// 事实上，指明为该 结构体实现的是 Fn、FnMut、或 FnOnce 中的哪种 trait，
// 对于约束该结构体的 类型而言就已经足够了。

// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn apply_closure() {
    let x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}

// 既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个 接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数。
// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

// Fn、FnMut 和 FnOnce 这些 trait 明确了闭包如何从周围的作用域 中捕获变量。
fn apply_function() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

fn main() {
    apply_closure();
    apply_function();
}