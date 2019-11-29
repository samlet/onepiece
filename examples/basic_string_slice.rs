//⊕ [Slice 类型 | Rust 中文](https://rustlang-cn.org/office/rust/book/understanding-ownership/ch04-03-slices.html)
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_cl() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    println!("the first word is: {}", word);
}

fn slice_cl(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    // 这类似于引用整个 String 不过带有额外的 [0..5] 部分。它不是对整个 String 的引用，而是对部分 String 的引用。start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range。如果需要包含 end，可以使用 ..= 而不是 ..：
    let hello2 = &s[0..=4];
    let world2 = &s[6..=10];

    // 对于 Rust 的 .. range 语法，如果想要从第一个索引（0）开始，
    // 可以不写两个点号之前的值。换句话说，如下两个语句是相同的：
    let slice = &s[0..2];
    let slice = &s[..2];

    // 如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。这意味着如下也是相同的：
    let s = String::from("hello");
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // 也可以同时舍弃这两个值来获取整个字符串的 slice。所以如下亦是相同的：
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_cl() {
    let mut s = String::from("hello world");
    let word = first_word_slice(&s);

    // s.clear(); // error!
    // 当拥有某值的不可变引用时，就不能再获取一个可变引用。
    // 因为 clear 需要清空 String，它尝试获取一个可变引用，它失败了。

    println!("the first word is: {}", word);
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str_cl() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word_str(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word_str(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word_str(my_string_literal);
}

fn main() {
    let s = "Hello, world!";
    // 这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

    first_word_cl();
    slice_cl();
    first_word_slice_cl();
    first_word_str_cl();
}