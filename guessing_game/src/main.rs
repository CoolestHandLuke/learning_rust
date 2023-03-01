// Program will generate a random number between 1 and 100. User will input a guess, 
// program will determine if it is correct, too high, or too low, and output the 
// appropriate output to the user. Exits when the guess is correct

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed {guess}");
}
