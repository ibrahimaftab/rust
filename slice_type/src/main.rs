fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    // let hello = &s[0..4];
    // let world = &s[6..10];

    // println!("{} {}", hello, world);
    println!("{}", word);

    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
