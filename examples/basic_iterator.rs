//⊕ [Iterator::any - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/closure_examples/iter_any.html)
//Iterator::any 是一个函数，若传给它一个迭代器（iterator），当其中任一元素满足 谓词（predicate）时它将返回 true，否则返回 false（译注：谓词是闭包规定 的， true/false 是闭包作用在元素上的返回值）。
/*
pub trait Iterator {
    // 被迭代的类型。
    type Item;

    // `any` 接受 `&mut self` 参数（译注：回想一下，这是 `self: &mut Self` 的简写）
    // 表明函数的调用者可以被借用和修改，但不会被消耗。
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
        // `Self::Item` 指明了被捕获变量的类型（译注：是迭代器的元素本身的类型）
        F: FnMut(Self::Item) -> bool {}

        // 译注：原文说 `Self::Item` 表明变量是通过值传递给闭包的，这是说错了。
        // `FnMut` 就表示闭包只能通过引用捕获变量。把类型为 `T` 的变量作为闭包
        // 的参数不代表闭包会拿走它的值，也可能是拿走它的引用。
}

*/
fn it_any_cl() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

// ⊕ [Iterator::find - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/closure_examples/iter_find.html)
/*
Iterator::find 是一个函数，在传给它一个迭代器时，将用 Option 类型返回第一个 满足谓词的元素。它的签名如下：
pub trait Iterator {
    // 被迭代的类型。
    type Item;

    // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
    // 但不会被消耗。
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
        // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型）
        P: FnMut(&Self::Item) -> bool {}
}
*/
fn it_find_cl() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}

fn main() {
    it_any_cl();
    it_find_cl();
}