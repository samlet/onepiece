/**
在过去24小时内修改过的文件名
通过调用获取当前工作目录env::current_dir，然后对于每个条目fs::read_dir，
通过提取 DirEntry::path和获取元数据fs::Metadata。
在 Metadata::modified返回SystemTime::elapsed上次修改时间。
Duration::as_secs将时间转换为秒，并与24小时（24 * 60 * 60秒）进行比较。
Metadata::is_file过滤掉目录。
*/

use std::{env, fs, io};
//use std::error::Error;

fn run() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        // let last_modified = metadata.modified()?.elapsed()?.as_secs();
        let last_modified = metadata.modified()?.elapsed().unwrap().as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")
            );
        }
    }

    Ok(())
}

fn main() {
    run();
}
