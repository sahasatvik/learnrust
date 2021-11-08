use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random secret number
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        // Get a guess from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the guess into an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Make sure the guess is in range
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
