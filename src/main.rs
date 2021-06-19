use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let mut f = File::open(path).unwrap();
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
    [algorithm0, algorithm1, algorithm2]
        .iter()
        .enumerate()
        .for_each(|(index, algorithm)| {
            let start = SystemTime::now();
            algorithm(&words);
            let time_spent = SystemTime::now().duration_since(start).unwrap();

            println!("Algorithm {} spent {:?}μs.", index, time_spent.as_micros(),);
        });
}

// Slowest 34ms 83k words
fn algorithm0(ref words: &Vec<String>) {
    let mut map = BTreeMap::new();
    for word in *words {
        match map.get_mut(word) {
            None => {
                map.insert(word, 1);
            }
            Some(value) => {
                *value += 1;
            }
        }
    }
}

// Second 18ms 83k words
fn algorithm1(ref words: &Vec<String>) {
    let mut map = BTreeMap::new();
    for word in *words {
        *map.entry(word).or_insert(0usize) += 1;
    }
}

// Faster 10ms 83k words
fn algorithm2(ref words: &Vec<String>) {
    let mut map = HashMap::new();
    for word in *words {
        *map.entry(word).or_insert(0usize) += 1;
    }
}
