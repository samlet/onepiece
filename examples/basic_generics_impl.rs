//⊕ [实现 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/generics/impl.html)

#![allow(dead_code)]
#![allow(unused_variables)]
fn some_defs() {
    struct S; // 具体类型 `S`
    struct GenericVal<T>(T,); // 泛型类型 `GenericVal`

    // GenericVal 的 `impl`，此处我们显式地指定了类型参数：
    impl GenericVal<f32> {} // 指定 `f32` 类型
    impl GenericVal<S> {} // 指定为上面定义的 `S`

    // `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
    impl <T> GenericVal<T> {}
}

struct Val {
    val: f64
}

struct GenVal<T>{
    gen_val: T
}

// Val 的 `impl`
impl Val {
    fn value(&self) -> &f64 { &self.val }
}

// GenVal 的 `impl`，指定 `T` 是泛型类型
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
