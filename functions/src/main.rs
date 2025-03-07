fn main() {
    println!("Hello, world!");
    another_function(1);
    print_label_measurement(10, 'h');
    println!("The value of the return value is: {}", return_value());
    println!("The value of the addition is: {}", addition(100))
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The value of the measurement is: {value} {unit_label}")
}

fn return_value() -> i32 {
    5
}

fn addition(x: i32) -> i32 {
    x + 1
}
