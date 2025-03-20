fn main() {
    loop {
        let mut input = String::new();

        println!("");
        println!("Please enter a palindrome number: \n");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trim = input.trim();

        println!("");

        match input.trim().parse::<u32>() {
            Ok(_) => {
                let string: &str = if check_palindrome(&trim) { "" } else { "not " };
                println!("The number is {}a palindrome", string);
            }
            Err(_) => {
                println!("Please type a valid number");
                continue;
            }
        };

        println!("");

        break;
    }
}

fn check_palindrome(number: &str) -> bool {
    let reverse: String = number.chars().rev().collect();
    return number == reverse;
}
