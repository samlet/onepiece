extern crate whatlang;

use whatlang::{detect, Lang, Script};

//⊕ [greyblake/whatlang-rs: Natural language detection library for Rust. Try demo online: https://www.greyblake.com/whatlang/](https://github.com/greyblake/whatlang-rs)

fn test_text(text: &str){
    let info = detect(text).unwrap();
    println!("lang: {}", info.lang());
}

fn main() {
    let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";

    let info = detect(text).unwrap();
    assert_eq!(info.lang(), Lang::Epo);
    assert_eq!(info.script(), Script::Latin);
    assert_eq!(info.confidence(), 1.0);
    assert!(info.is_reliable());

    test_text("这是中文");
    test_text("私はアメリカ人です");
    test_text("J'aimerais être marié.")
}
