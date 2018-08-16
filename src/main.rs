extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let answer = rand::thread_rng().gen_range(1, 101);

    println!("answer is: {}", answer);

    println!("Please enter your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid entry.");

    let guess: u8 = guess.trim().parse().expect("Please enter a number.");

    println!("You guessed: {}", guess);

    match guess.cmp(&answer) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct!\nYou Win!"),
    }
}
