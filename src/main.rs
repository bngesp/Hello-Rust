mod word_count;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::borrow::Borrow;

fn main(){

    let filename = "/Users/admin/Rust/Hello-rust/hello-rust/test.txt".to_string();
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_counter = word_count::WordCount::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display(2u64.borrow());
}