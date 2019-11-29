/*
模块可以分配到文件/目录的层次结构中。让我们将《可见性》一节中 的例子的代码拆分到多个文件中：
⊕ [文件分层 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/mod/split.html)

$ tree .
.
|-- my
|   |-- inaccessible.rs
|   |-- mod.rs
|   `-- nested.rs
`-- split.rs
*/
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。
// 如果 some_file.rs 里面含有 mod 声明，那么模块文件的内容将在编译之前被插入 crate 文件的相应声明处。换句话说，模 块不会单独被编译，只有 crate 才会被编译。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

