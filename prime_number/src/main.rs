use std::io;

fn main() {
    println!("\nPrime number checker!\n");

    loop {
        println!("Please enter the number you want to check: \n");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("\nFailed to read line\n");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a number\n");
                return;
            }
        };

        prime_check(number);

        println!("\nWould you like to check another number? (y/n)\n");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("\nFailed to read line\n");

        if input.trim().to_lowercase() != "y" {
            break;
        } else {
            println!("\n");
        }
    }
}

fn prime_check(n: i32) {
    let message = if n < 2 {
        format!("{} is not a prime number", n)
    } else if n == 2 || n == 3 {
        format!("{} is a prime number", n)
    } else if n % 2 == 0 || n % 3 == 0 {
        format!("{} is not a prime number", n)
    } else {
        let mut i = 5;
        let mut is_prime = true;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                is_prime = false;
                break;
            }
            i += 6;
        }

        if is_prime {
            format!("{} is a prime number", n)
        } else {
            format!("{} is not a prime number", n)
        }
    };
    println!("{}", message);
}
