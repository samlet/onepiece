//⊕ [子进程 - 通过例子学 Rust](https://rustwiki.org/zh-CN/rust-by-example/std_misc/process.html)
//process::Output 结构体表示已结束的子进程（child process）的输出，而 process::Command 结构体是一个进程创建者（process builder）。
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn basic_cl() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}

static PANGRAM: &'static str =
    "the quick brown fox jumped over the lazy dog\n";

fn pipe_cl() {
    // 启动 `wc` 命令
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // 将字符串写入 `wc` 的 `stdin`。
    //
    // `stdin` 拥有 `Option<ChildStdin>` 类型，不过我们已经知道这个实例不为空值，
    // 因而可以直接 `unwrap 它。
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // 因为 `stdin` 在上面调用后就不再存活，所以它被 `drop` 了，管道也被关闭。
    //
    // 这点非常重要，因为否则 `wc` 就不会开始处理我们刚刚发送的输入。

    // `stdout` 字段也拥有 `Option<ChildStdout>` 类型，所以必需解包。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

fn wait_cl() {
    println!("{}", "wait 2 secs ...");
    let mut child = Command::new("sleep").arg("2").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}

fn main() {
    basic_cl();
    pipe_cl();
    wait_cl();
}