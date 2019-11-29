extern crate rust_stemmers;

use rust_stemmers::{Algorithm, Stemmer};

fn main() {
    let en_stemmer = Stemmer::create(Algorithm::English);
    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");

    println!("{}", en_stemmer.stem("dogs"));

    french_test();
}

fn stemms_to(lhs: &str, rhs: &str, stemmer: Algorithm) {
    assert_eq!(Stemmer::create(stemmer).stem(lhs), rhs);
}

fn french_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_fr.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_fr.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::French);
    }
}
