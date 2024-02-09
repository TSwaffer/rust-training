use std::io;
use rand::{self, Rng};

fn main() {
  let random_number: u16 = rand::thread_rng().gen_range(1..101);

  let mut buffer: String = String::new();
  println!("Enter a number:");
  io::stdin().read_line(&mut buffer);
  let mut number: u16 = buffer.trim().parse().unwrap();

  while number != random_number {
    buffer = String::new();

    if number < random_number {
      println!("Too low!");
    } else {
      println!("Too high!");
    }

    println!("Try again. Enter a number:");
    io::stdin().read_line(&mut buffer);
    number = buffer.trim().parse::<u16>().unwrap();
  }

  print!("You got it!!");
}