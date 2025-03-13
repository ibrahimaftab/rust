fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    // let hello = &s[0..4];
    // let world = &s[6..10];
    // println!("{} {}", hello, world);
    println!("{}", word);

    let arr = [1, 2, 3, 4, 5, 6];

    let slice = &arr[1..3];

    assert_eq!(slice, &[2, 3]);

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
