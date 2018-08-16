extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let answer = rand::thread_rng().gen_range(1, 101);

    println!("answer is: {}", answer);

    println!("Please enter your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid entry.");

    println!("You guessed: {}", guess);
}
