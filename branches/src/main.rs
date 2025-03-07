fn main() {
    let number = 5;

    // if number < 5 {
    //     println!("The condition was true");
    // } else if number < 10 {
    //     println!("The number is less than 10");
    // } else {
    //     println!("The condition was false");
    // }

    // Shorthand
    let boolean = if number < 5 {
        true
    } else if number < 10 {
        false
    } else {
        false
    };

    println!("The value of boolean is: {boolean}");
}
