use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ Configuration

const MAX_GUESSES: u8 = 10;
const SECRET_MIN: u8 = 1;
const SECRET_MAX: u8 = 100;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ Implementation

fn print_welcome() {
    println!("Guessing game time!\n");
}

fn remaining_guesses(current_guess_number: u8) -> Option<u8> {
    if current_guess_number > MAX_GUESSES {
        return None;
    }

    MAX_GUESSES.checked_sub(current_guess_number)
}

fn request_guess(current_guess_number: u8) -> Result<u8, ParseIntError> {
    let mut guess = String::new();

    println!("Please input your {current_guess_number}. guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse::<u8>()
}

fn check_guess(guess: u8, secret: u8, current_guess_number: u8) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less => {
            println!("💡 Too low!");

            false
        }
        Ordering::Equal => {
            println!();
            if current_guess_number > 1 {
                println!("🎉🎉🎉 You found the secret with {current_guess_number} guesses 🎉🎉🎉");
            } else {
                println!("🎉🎉🎉 You found the secret with only a single guess - nice! 🎉🎉🎉");
            }
            println!();

            true
        }
        Ordering::Greater => {
            println!("💡 Too high!");

            false
        }
    }
}

fn main() {
    let secret = rand::rng().random_range(SECRET_MIN..=SECRET_MAX);
    let mut current_guess_number: u8 = 0;

    print_welcome();

    loop {
        // Remaining guesses
        current_guess_number = current_guess_number.strict_add(1);
        let guesses_left = remaining_guesses(current_guess_number);
        if guesses_left.is_none() {
            println!(
                "💀💀💀 You guessed {current_guess_number} times... and failed. You loose! 💀💀💀"
            );
            break;
        }
        let guesses_left = guesses_left.unwrap();

        // Input
        let guess = match request_guess(current_guess_number) {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️️ Please provide a number between 1 and 100!\n");
                continue;
            }
        };

        // Value handling
        if check_guess(guess, secret, current_guess_number) {
            break;
        }

        match guesses_left.cmp(&1) {
            Ordering::Greater => println!("{guesses_left} tries left... try again!"),
            Ordering::Equal => println!("⚠️️ Only one try left... good luck!"),
            Ordering::Less => println!("No more try left."),
        }
        print!("\n\n");
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ Tests

#[cfg(test)]
mod tests {
    #[test]
    fn arrays() {
        // given
        let x: [u8; 4] = [1, 2, 3, 98];

        // when
        let y = x.iter().all(|&x| x <= 100);

        // then
        assert_eq!(y, true);
    }

    #[test]
    fn overflows() {
        // given
        let x: u8 = 250;
        let y: u8 = 1;
        let mut z: u8 = x;

        // when
        for _ in 0..=9 {
            z = z.wrapping_add(y);
        }

        // then
        assert_eq!(z as u16, ((x as u16 + 10) % 256));
    }

    #[test]
    fn for_expression() {
        // given
        let mut i: u8 = 0;

        // when
        let y = loop {
            i += 1;
            if i % 13 == 0 {
                break i;
            } else if i % 3 == 0 {
                break i + 1;
            }
        };

        // then
        assert_eq!(y, 4);
    }

    fn borrow_string(value: &String) -> usize {
        value.len()
    }

    #[test]
    fn test_strings() {
        // given
        let x = "This is a string literal";
        let mut y = String::from("This is a string instance");
        let z = y.clone();

        // when
        y.push_str("; appended.");
        let size = borrow_string(&y);

        // then
        assert_eq!(x, "This is a string literal");
        assert_eq!(y, "This is a string instance; appended.");
        assert_eq!(size, 36);
        assert_eq!(z, "This is a string instance");
    }
}
