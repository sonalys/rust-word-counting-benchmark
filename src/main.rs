use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    let mut f = File::open("dummy.txt").unwrap();
    let mut buffer = String::new();

    // Read entire file to buffer.
    f.read_to_string(&mut buffer).unwrap();

    // Split buffer by white space, lower case each word and convert to vector.
    let words = buffer
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>();

    println!("Text has {} words in total", words.len());

    // Runs each algorithm and compares run time.
    [algorithm0, algorithm1]
        .iter()
        .enumerate()
        .for_each(|(index, algorithm)| {
            let start = SystemTime::now();
            let _ = algorithm(&words);
            let time_spent = SystemTime::now().duration_since(start).unwrap();

            println!("Algorithm {} spent {:?}μs", index, time_spent.as_micros());
        });
}

fn algorithm0(ref words: &Vec<String>) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in *words {
        match map.get_mut(word) {
            None => {
                map.insert(word.to_string(), 1);
            }
            Some(value) => {
                *value += 1;
            }
        }
    }

    map
}

fn algorithm1(ref words: &Vec<String>) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in *words {
        *map.entry(word.to_string()).or_insert(0usize) += 1;
    }

    map
}
