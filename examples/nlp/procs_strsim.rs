// ⊕ [dguo/strsim-rs: Rust implementations of string similarity metrics](https://github.com/dguo/strsim-rs)

extern crate strsim;

use strsim::{hamming, levenshtein, normalized_levenshtein, osa_distance,
             damerau_levenshtein, normalized_damerau_levenshtein, jaro,
             jaro_winkler};
use strsim::generic_levenshtein;

fn main() {
    match hamming("hamming", "hammers") {
        Ok(distance) => assert_eq!(3, distance),
        Err(why) => panic!("{:?}", why)
    }

    assert_eq!(levenshtein("kitten", "sitting"), 3);

    assert!((normalized_levenshtein("kitten", "sitting") - 0.571).abs() < 0.001);

    assert_eq!(osa_distance("ac", "cba"), 3);

    assert_eq!(damerau_levenshtein("ac", "cba"), 2);

    assert!((normalized_damerau_levenshtein("levenshtein", "löwenbräu") - 0.272).abs() <
        0.001);

    assert!((jaro("Friedrich Nietzsche", "Jean-Paul Sartre") - 0.392).abs() <
        0.001);

    assert!((jaro_winkler("cheeseburger", "cheese fries") - 0.911).abs() <
        0.001);

    println!("{}", generic_levenshtein(&[1, 2, 3], &[0, 2, 5]));
}
