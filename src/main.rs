use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let max_tries = 10;
    let secret = rand::rng().random_range(1..=100);
    println!("Guessing game time!");

    let mut tries: u8 = 0;
    let mut guess = String::new();

    while tries < max_tries {
        // Reset
        guess.clear();

        // Input
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = guess.trim().parse().expect("Please type a number between 1 and 100!");

        // Value handling
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too low!");
            }
            Ordering::Equal => {
                println!("You guessed the secret - nice!");
                break;
            }
            Ordering::Greater => {
                println!("Too high!");
            }
        }

        // Remaining tries
        tries += 1;
        if tries < max_tries {
            println!("That was your {tries}. guess... try again!");
            continue;
        }

        println!("You had your tries... and failed.");
    }
}
