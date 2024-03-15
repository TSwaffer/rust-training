use std::{io, num::ParseIntError};
use rand::prelude::*;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..101);

  println!("I'm thinking of a number between 1 and 100");
  println!("Guess the number:");

  loop {
      let mut guess = String::new();
      let _ = io::stdin().read_line(&mut guess);
      let parsed_guess: Result<u32, ParseIntError> = guess.trim().parse();
      let guess = match parsed_guess {
        Ok(entry) => entry,
        Err(error) => {
          println!("Error occurred: {:?}", error);
          continue;
        }
      };
      
      if guess > secret_number {
        println!("\n{} is too high! Guess lower:", guess);
      } else if guess < secret_number {
        println!("\n{} is too low! Guess higher:", guess);
      } else {
        println!("\nYou got it! The secret number was {}", secret_number);
        break;
      }
  }
}