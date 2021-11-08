use std::io;

fn main() {
    println!("Enter a positive integer.");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let mut number: u64 = number
        .trim()
        .parse()
        .expect("Not a positive integer");

    let mut factorial: u64 = 1;

    while number > 1 {
        factorial *= number;
        number -= 1;
    }

    println!("The factorial is {}.", factorial);
}
