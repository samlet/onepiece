use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::error::Error;

enum MyError {
    FileWriteError
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::FileWriteError
    }
}

// The preferred method of quick returning Errors
fn write_to_file_question() -> Result<(), MyError> {
    let mut file = File::create("my_best_friends.txt")?;
    file.write_all(b"This is a list of my best friends.")?;
    Ok(())
}

// The previous method of quick returning Errors
fn write_to_file_using_try() -> Result<(), MyError> {
    let mut file = r#try!(File::create("my_best_friends.txt"));
    r#try!(file.write_all(b"This is a list of my best friends."));
    Ok(())
}

// This is equivalent to:
fn write_to_file_using_match() -> Result<(), MyError> {
    let mut file = r#try!(File::create("my_best_friends.txt"));
    match file.write_all(b"This is a list of my best friends.") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }
    Ok(())
}

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
fn basic_cl(){
    let path = Path::new("data/lorem_ipsum.txt");
    let display = path.display();

    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main(){
    write_to_file_question();
    basic_cl();
}
