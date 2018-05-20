use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::env;
use std::fs::File;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    let first_arg:Vec<String> = env::args().collect();
    let file_path = &first_arg[1];
    println!("trying to read {}", file_path);
    let file = File::open(file_path).unwrap();

    let mut map : HashMap<String,i32> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line_val = line.expect("failed to read line");
        let val = match map.entry(line_val) {
            Vacant(entry) => entry.insert(0),
            Occupied(entry) => entry.into_mut(),
        };
        *val += 1;
    }

    for (entry, count) in map.iter() {
        println!("line: {} appears: {}", entry, count);
    }
}
