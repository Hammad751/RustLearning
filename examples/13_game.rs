#![allow(unused)]

use std::cmp::Ordering;
use std::{io, println};
fn main() {
    println!("Guessing game!");

    let random_guess = rand::random_range(1..=10); // gen_range generates a random number between 1 and 10 inclusive

    let mut attempts = 3;

    loop {
        if attempts == 0 {
            println!("You lose!");
            break;
        }
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        println!("Random guess: {}", random_guess);
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // trim removes whitespace from the string and parse converts the string to a number

        let trimmed_guess = guess.trim();

        // Check for exit commands
        // If the user types "quit", "exit", or "q" (case-insensitive), the game will exit gracefully
        if(trimmed_guess.eq_ignore_ascii_case("quit") || 
            trimmed_guess.eq_ignore_ascii_case("exit") || 
            trimmed_guess.eq_ignore_ascii_case("q")) {
            println!("Exiting the game.");
            break;
        }

        // Errr handling for non-numeric input
        let guess: u32 = match trimmed_guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // Compare the guess with the random number and provide feedback
        match guess.cmp(&random_guess) {

            // If the guess is equal to the random number, the player wins and the game ends
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too large!"),
        }

        attempts -= 1;
        println!("Try Again");
    }
}
