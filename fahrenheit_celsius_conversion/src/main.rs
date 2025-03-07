fn main() {
    loop {
        println!("Enter the temperature in fahrenheit");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number");
                continue;
            }
        };

        println!("input: {}", input);

        let celsius = (input - 32.0) * 5.0 / 9.0;
        println!("{:.2}", celsius);
        break;
    }
}
