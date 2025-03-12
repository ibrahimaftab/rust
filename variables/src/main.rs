fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = &mut x;

    println!("The value of y is: {y}");

    let test = "demo";
    println!("The value of test is: {test}");
    {
        let test = 100;
        println!("The value of test is: {test}");
    }
    println!("The value of test is: {test}");
    println!("\n");

    const DEMO: &str = "demo";
    println!("The value of demo is: {DEMO}");
    println!("\n");

    let obj = (1, 2, 3);
    let (a, b, c) = obj;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    println!("\n");

    let obj = [1, 2, 3];
    let [a, b, c] = obj;
    let d = obj[1];
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");
    println!("Getting value directly: {d}");

    println!("\n");

    let nums = [5; 5];
    let [a, b, c, d, e] = nums;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");
    println!("The value of d is: {d}");
    println!("The value of d is: {e}");

    let mut string = String::from("Hello");
    println!("The value of string is: {string}");
    let string2 = &mut string;
    string2.push_str(" world");
    // println!("The value of string is: {string}");
    println!("The value of string2 is: {string2}");

    let mut string3 = String::from("Happy");

    let output = change(&mut string3);

    println!("{}", output);
}

fn change(s: &mut String) -> &String {
    s.push_str(" Folks");
    s
}
