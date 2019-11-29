use std::fs::File;
use std::io::Read;
use std::path::Path;

// file_double从文件中读取内容并将其转化成i32类型再翻倍。
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

// 使用传统的match和if let来改写它
fn file_double_2<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

//try!事实上就是match Result的封装，当遇到Err(E)时会提早返回，
// ::std::convert::From::from(err)可以将不同的错误类型返回成最终需要的错误类型，
// 因为所有的错误都能通过From转化成Box<Error>，所以下面的代码是正确的：

use std::error::Error;
//use std::fs::File;
//use std::io::Read;
//use std::path::Path;

fn file_double_3<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = r#try!(File::open(file_path));
    let mut contents = String::new();
    r#try!(file.read_to_string(&mut contents));
    let n = r#try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}
fn file_double_4<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

fn main() {
    match file_double("data/foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
    match file_double_2("data/foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
    match file_double_3("data/foobar_absent") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
    match file_double_4("data/foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}

