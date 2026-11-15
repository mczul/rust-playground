use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let max_tries = 10;
    let secret_min: u8 = 1;
    let secret_max: u8 = 100;
    let secret = rand::rng().random_range(secret_min..=secret_max);
    println!("Guessing game time!");

    let mut tries: u8 = 0;
    let mut guess = String::new();

    loop {
        // Reset
        guess.clear();

        // Input
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = match guess.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a number between 1 and 100!");
                continue;
            }
        };

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
        if tries >= max_tries {
            println!("You tried {tries} times and failed. You loose!");
            break;
        }

        let tries_left = max_tries - tries;
        match tries_left.cmp(&1) {
            Ordering::Greater => println!("{tries_left} tries left... try again!"),
            Ordering::Equal => println!("Only one try left... good luck!"),
            Ordering::Less => println!("No more try left."),
        }
    }
}
