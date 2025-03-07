fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result} \n");
    nested_loop();
    while_loop();
    for_loop();
    range_reverse_loop();
}

fn nested_loop() {
    let mut counter = 0;
    'counter_up: loop {
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counter_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("counter_up = {counter} \n");
}

fn while_loop() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    println!("\n");
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5, 6, 7];

    for element in a {
        println!("The value is: {}", element);
    }
    println!("\n");
}

fn range_reverse_loop() {
    let a = [1, 2, 3, 4, 5, 6, 7];

    for element in (0..5).rev() {
        println!("The value is: {}", element);
        println!("The value is: {}", a[element]);
    }
    println!("\n");
}
