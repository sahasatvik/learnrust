use std::io;

fn main() {
    let mut largest = i64::MIN;

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    for word in line.split_whitespace() {
        let n: i64 = word.parse().expect("Not a number");
        if n > largest {
            largest = n;
        }
    }

    println!("{}", largest);
}
