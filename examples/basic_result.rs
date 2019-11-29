fn double_number(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

use std::num::ParseIntError;

fn double_number_2(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

use std::env;

// 可以在值为None的时候返回一个Result::Err(E)，值为Some(T)的时候返回Ok(T)，
// 利用它我们可以组合Option和Result：
fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn proc_ascii(){
    let ascii = "hello!\n";
    let non_ascii = "Grüße, Jürgen ❤";

    assert!(ascii.is_ascii());
    assert!(!non_ascii.is_ascii());
}

fn main() {
    let n: i32 = double_number("10");
    assert_eq!(n, 20);

    match double_number_2("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }

    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }

    proc_ascii();
    t_early_returns();
}

//⊕ [提前返回 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/error/result/early_returns.html)
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn t_early_returns() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
