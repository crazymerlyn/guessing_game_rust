use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess:");

    let input = io::stdin().read_line().ok().expect("Failed to read input!");
    println!("You guessed: {}", input);
}
