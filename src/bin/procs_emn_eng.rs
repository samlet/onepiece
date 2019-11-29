use std::fs::File;
use std::io::{BufRead, BufReader};
//use difflib::differ::Differ;
//use difflib::sequencematcher::SequenceMatcher;
use fst::{IntoStreamer, Set};
use fst_levenshtein::Levenshtein;
use std::error::Error;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn use_diff(key_words: &Vec<String>, value_words: &Vec<String>) -> Result<(), Box<Error>> {
    let lookups: Vec<&str> = key_words.iter().map(|s| &**s).collect();

    println!("{}", "search a expr ...");
    let now = Instant::now();
    let result = difflib::get_close_matches("i am student", lookups, 3, 0.6);
    println!(".. query cost {} secs and {} ms: {:?}", now.elapsed().as_secs(), now.elapsed().subsec_micros(), result);
    for el in result {
        let index = key_words.iter().position(|r| r == el).unwrap();
        println!("{} - {} / {}", index, el, value_words[index]);
    }

    Ok(())
}

fn use_fst(key_words: &Vec<String>) ->  Result<(), Box<Error>> {
    let mut lookups: Vec<&str> = key_words.iter().map(|s| &**s).collect();
    lookups.sort();
    let set = Set::from_iter(lookups)?;

    // Build our fuzzy query.
    let lev = Levenshtein::new("Hi.", 1)?;

    // Apply our fuzzy query to the set we built.
    let stream = set.search(lev).into_stream();
    let keys = stream.into_strs()?;
    println!("result {:?}", keys);

    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    // let prefix="/pi/ai/seq2seq/cmn-eng";
    // let file = File::open(prefix.to_owned()+"/cmn.txt")?;

    // let path="/pi/ai/seq2seq/cmn-eng/cmn.txt";
    let path="/pi/ai/seq2seq/jpn-eng/jpn.txt";

    let now = Instant::now();
    let file = File::open(path)?;
    let mut total=0;
    let mut key_words:Vec<String> = vec![];
    let mut value_words:Vec<String>=vec![];

    let f = BufReader::new(file);
    for line in f.lines() {
        let s=line.unwrap().clone();
        let parts: Vec<&str> = s.split("\t").collect();
        key_words.push(parts[0].into());
        value_words.push(parts[1].into());
        total += 1;
    }
    println!(".. load file cost {} secs and {} ms, total lines {}",
             now.elapsed().as_secs(), now.elapsed().subsec_micros(), total);
    use_diff(&key_words, &value_words)
    // use_fst(&key_words)
}
