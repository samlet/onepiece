//⊕ [死代码 dead_code - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/attribute/unused.html)
fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
