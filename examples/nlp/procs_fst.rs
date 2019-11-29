extern crate fst;
extern crate fst_levenshtein;

use std::error::Error;
use std::process;

use fst::{IntoStreamer, Set};
use fst_levenshtein::Levenshtein;

//⊕ [BurntSushi/fst: Represent large sets and maps compactly with finite state transducers.](https://github.com/BurntSushi/fst)
//⊕ [fst - Rust](https://docs.rs/fst/0.3.3/fst/)

fn try_main() -> Result<(), Box<Error>> {
    // A convenient way to create sets in memory.
    let keys = vec!["fa", "fo", "fob", "focus", "foo", "food", "foul"];
    let set = Set::from_iter(keys)?;

    // Build our fuzzy query.
    let lev = Levenshtein::new("foo", 1)?;

    // Apply our fuzzy query to the set we built.
    let stream = set.search(lev).into_stream();

    let keys = stream.into_strs()?;
    assert_eq!(keys, vec!["fo", "fob", "foo", "food"]);
    Ok(())
}


use fst::{Streamer};
use fst::set;
use fst_regex::Regex;

fn regex_cl() -> Result<(), Box<Error>> {
    let set1 = Set::from_iter(&["AC/DC", "Aerosmith"])?;
    let set2 = Set::from_iter(&["Bob Seger", "Bruce Springsteen"])?;
    let set3 = Set::from_iter(&["George Thorogood", "Golden Earring"])?;
    let set4 = Set::from_iter(&["Kansas"])?;
    let set5 = Set::from_iter(&["Metallica"])?;

    // Create the regular expression. We can reuse it to search all of the sets.
    let re = Regex::new(r".+\p{Lu}.*")?;

    // Build a set operation. All we need to do is add a search result stream for
    // each set and ask for the union. (Other operations, like intersection and
    // difference are also available.)
    let mut stream =
        set::OpBuilder::new()
            .add(set1.search(&re))
            .add(set2.search(&re))
            .add(set3.search(&re))
            .add(set4.search(&re))
            .add(set5.search(&re))
            .union();

    // Now collect all of the keys. Alternatively, you could build another set here
    // using `SetBuilder::extend_stream`.
    let mut keys = vec![];
    while let Some(key) = stream.next() {
        keys.push(key.to_vec());
    }
    assert_eq!(keys, vec![
        "AC/DC".as_bytes(),
        "Bob Seger".as_bytes(),
        "Bruce Springsteen".as_bytes(),
        "George Thorogood".as_bytes(),
        "Golden Earring".as_bytes(),
    ]);
    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }

    if let Err(err)=regex_cl(){
        eprintln!("{}", err);
        process::exit(1);
    }
}
