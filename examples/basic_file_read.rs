use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn basic_cl(){
    // 创建指向所需的文件的路径
    let path = Path::new("my_best_friends.txt");
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} contains:\n{}\n", display, s),
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
}

fn main() -> io::Result<()> {
    basic_cl();

    let mut f = File::open("my_best_friends.txt")?;
    let mut buffer = String::new();

    let result=f.read_to_string(&mut buffer)?;
    println!("{}: {}", result, buffer);

    Ok(())
}
