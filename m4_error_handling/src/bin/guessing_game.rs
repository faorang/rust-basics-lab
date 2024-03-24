#![allow(unused)]

use rand::Rng;
use std::{cmp::Ordering, io};

mod guessing_game {
    use super::*;

    pub fn play() {
        println!("Guess the number! (1-10)");

        let secret_number = rand::thread_rng().gen_range(1..=10);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Guess value must be a number.");
                    continue;
                }
            };

            if !(1..=10).contains(&guess) {
                println!("The secret number will be between 1 and 10.");
                continue;
            }

            println!("You guessed: {guess}");

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
}


fn main() {
    let _ = guessing_game::play();
}

/// # Exercise
/// 
/// Refactor the guessing game to use a custom error type for error handling.
/// 
/// # Hints
/// 
/// - Create a custom error type that can hold an error message.
/// - Implement the `From` trait for the custom error type to convert from `std::io::Error` and `String`.
/// - Change the return type of the `play` function to `Result<(), MyError>`.
/// - Use the `?` operator to propagate errors.
/// - Change the error handling in the `play` function to use the custom error type.
/// - Change the error handling in the `main` function to use the custom error type.
/// - Add a test to check if the `play` function returns `
///     - Ok(())` when the user guesses the correct number.
///     - an error when the system fails to read user input.
///     - an error when the user enters a non-numeric guess (i.e., parse error).
///     - an error when the user enters a guess that is out of range.
/// 
mod guessing_game_custom_error {
    enum MyError {
        IoError(std::io::Error),
        ParseError(String),
        RangeError(String),
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn should_return_io_error_when_fails_to_read() {
            
        }

        #[test]
        fn should_return_parse_error_if_input_is_non_numeric() {
            
        }

        #[test]
        fn should_retun_out_of_range_error_if_input_is_not_in_range() {
            
        }
    }
}

