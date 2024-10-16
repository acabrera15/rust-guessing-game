use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn yes_or_no() -> bool {
    let mut input = String::new();
    loop {
        print!("Please enter y/n: ");
        io::stdout().flush().expect("Failed to flush output");

        input.clear(); // Clear previous input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input, please enter 'y' or 'n'."),
        }
    }
}

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("enter your guess");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("There was an issue reading the line");

            // convert to number to match secret number
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number");
                    continue;
                }
            };

            println!("Your guess was: {guess}");

            // handle compare options
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too large!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

        println!("Would you like to play again? (y/n)");

        let input = yes_or_no();

        if !input {
            print!("Thank you for playing. Come back again.");
            break;
        }
    }
}
