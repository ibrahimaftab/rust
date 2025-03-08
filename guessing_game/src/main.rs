use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let level = difficulty();
    guess_game(level);
}

fn difficulty() -> u32 {
    println!("\n\n-----------------\nGuess the number! \n-----------------\n");

    loop {
        println!("Choose the difficulty level:\n");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard \n");

        let mut difficulty = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line!");

        println!();

        match difficulty.trim().parse() {
            Ok(1) => {
                println!("You chose Easy!");
                return 100;
            }
            Ok(2) => {
                println!("You chose Medium!");
                return 250;
            }
            Ok(3) => {
                println!("You chose Hard!");
                return 500;
            }
            _ => {
                println!("Please choose a valid difficulty level!");
                continue;
            }
        }
    }
}

fn guess_game(level: u32) {
    println!("\nPlease input your guess.");

    let mut attempts: u32 = 5;

    let secret_number = rand::thread_rng().gen_range(1..=level);

    while attempts > 0 {
        println!("\nYou have {} attempts left.", attempts);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }

        attempts -= 1;
    }

    println!("\n ❌ You lose! The secret number was: {}", secret_number);
}
