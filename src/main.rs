use std::io;
use rand::prelude::*;

fn main() {
    let max_tries = 10;
    let secret = rand::rng().random_range(1..=100);
    println!("Guessing game time!");

    let mut tries: u8 = 0;
    let mut user_input = String::new();

    while tries < max_tries {
        // Reset
        user_input.clear();

        // Input
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let trimmed_user_input = user_input.trim();
        println!("You guessed: {trimmed_user_input}");

        // Success
        if trimmed_user_input == format!("{secret}") {
            println!("You guessed the secret - nice!");
            break;
        }

        // Failure
        tries += 1;
        if tries < max_tries {
            println!("That was your {tries}. guess... try again!");
            continue;
        }

        println!("You had your tries... and failed.");
    }
}
