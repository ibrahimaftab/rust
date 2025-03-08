use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nLet's Play Rock Paper Scissors\n");

    let mut attempts = 3;
    let mut myscore = 0;
    let mut yourscore = 0;

    while attempts > 0 {
        let mut sign = String::new();
        println!("You have {} chances to beat me", attempts);

        let random_sign = rand::thread_rng().gen_range(1..=3);
        let signs = ["Rock", "Paper", "Scissor"];

        println!("\nLet's play:");
        println!("Type '1' for Rock 🪨 and enter");
        println!("Type '2' for Paper 📜 and enter");
        println!("Type '3' for Scissor ✂️ and enter\n");

        io::stdin()
            .read_line(&mut sign)
            .expect("\n❌ Failed to read input!\n");

        let choice: usize = match sign.trim().parse() {
            Ok(num) if (1..=3).contains(&num) => num,
            _ => {
                println!("❌ Invalid choice! Please enter 1, 2, or 3.");
                continue;
            }
        };

        println!("\nI picked {}\n", signs[random_sign - 1]);

        let status: &str = match (choice, random_sign) {
            (1, 1) | (2, 2) | (3, 3) => "It's a tie! 🤝",
            (1, 2) | (2, 3) | (3, 1) => {
                myscore += 1;
                "😈 You lose!"
            }
            (1, 3) | (2, 1) | (3, 2) => {
                yourscore += 1;
                "🎉 You win!"
            }
            _ => unreachable!(),
        };

        println!("{status}\n");

        if attempts == 1 {
            let final_status = match myscore.cmp(&yourscore) {
                Ordering::Equal => "It's a tie! 🤝",
                Ordering::Greater => "😈 You lose! better luck next time",
                Ordering::Less => "🎉 You won! Congratulations!",
            };

            println!("\nGame Over!\n");
            println!("{}", final_status);

            println!("\nWant to play again? Type 'yes' to continue or any key to exit.\n");

            let mut play_again = String::new();

            io::stdin()
                .read_line(&mut play_again)
                .expect("\n❌ Failed to read input!\n");

            let play_again = play_again.trim().to_lowercase();

            if play_again == "yes" {
                attempts = 3;
                myscore = 0;
                yourscore = 0;
                println!();
                continue;
            } else {
                println!("\nGoodbye!\n");
            }
        }

        attempts -= 1;
    }
}
