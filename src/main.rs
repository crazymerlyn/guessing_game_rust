extern crate rand;

use std::io;

fn main() {
    println!("Guess the number!");

    let mut tries_left = 7u32;
    let secret_number = (rand::random::<u32>() % 100u32) + 1u32;

    loop {
        println!("Please enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_num;

        match input.trim_right().parse::<u32>() {
            Ok(num) => input_num = num,
            Err(_)  => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", input);

        if input_num < secret_number {
            println!("Too small!")
        } else if input_num > secret_number {
            println!("Too big!")
        } else  {
            println!("You win");
            return;
        }

        tries_left -= 1u32;
        if tries_left == 0 {
            println!("You lost!");
            return;
        }
    }
}


