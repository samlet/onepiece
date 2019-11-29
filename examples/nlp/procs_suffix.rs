extern crate suffix;

use suffix::SuffixTable;

//⊕ [BurntSushi/suffix: Fast suffix arrays for Rust (with Unicode support).](https://github.com/BurntSushi/suffix)
fn main() {
    let st = SuffixTable::new("the quick brown fox was quick.");
    // assert_eq!(st.positions("quick"), vec![4, 24]);
    // println!("{}", st.positions("quick"));
    assert_eq!(st.positions("quick"), &[4, 24]);
}

/*
There is also a command line program, stree, that can be used to visualize suffix trees:

git clone git://github.com/BurntSushi/suffix
cd suffix/stree_cmd
cargo build --release
./target/release/stree "banana" | dot -Tpng | xv -
*/