use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut word_counts = HashMap::new();

    for (i, line) in lines.enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Malformed line {}", i);
                break;
            }
        };
        for word in line.split_whitespace() {
            let count = word_counts.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }

    println!("{:?}", word_counts);
}
