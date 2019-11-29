//⊕ [From 和 Into - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html)
//From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种 类型转换的简单机制。
// 在标准库中有无数 From 的实现，规定了原生类型及其他常见类 型的转换功能。
//
// Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，
// 那么同时你也就免费获得了 Into。
//使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
// 不过考虑到我们免费获得了 Into，这点代价不值一提。

#![allow(unused_variables)]

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);


    let num = Number::from(30);
    println!("My number is {:?}", num);

    t_into()
}

fn t_into() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

