use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    let mut f = File::open("dummy.txt").expect("could not open dummy.txt");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let words = buffer
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>();

    println!("Text has {} words in total", words.len());

    [algorithm1, algorithm2]
        .iter()
        .enumerate()
        .for_each(|(index, algorithm)| {
            let start = SystemTime::now();
            let _ = algorithm(&words);
            let time_spent = SystemTime::now().duration_since(start).unwrap();

            println!("Algorithm {} spent {:?}μs", index, time_spent.as_micros());
        });
}

fn algorithm1(ref words: &Vec<String>) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in *words {
        let lower_case = word.to_lowercase();
        match map.get_mut(&lower_case) {
            None => {
                map.insert(lower_case, 1);
            }
            Some(value) => {
                *value += 1;
            }
        }
    }

    map
}

fn algorithm2(ref words: &Vec<String>) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for word in *words {
        *map.entry(word.to_string()).or_insert(0usize) += 1;
    }

    map
}
