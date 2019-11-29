use std::ffi::{OsString, OsStr};
//⊕ [std::ffi::OsString - Rust](https://doc.rust-lang.org/std/ffi/struct.OsString.html)

fn main() {
    let os_string = OsString::from("foo");
    let os_str = OsStr::new("foo");
    assert_eq!(os_string.as_os_str(), os_str);
}
