fn main() {
    let word = String::from("haey");

    let reverse = reverse_word_manually(&word);
    println!("{}", reverse);

    let word2 = String::from("nhoJ");
    let reverse2 = reverse_word(&word2);

    println!("{}", reverse2);

    let words = String::from("world hello");
    let reverse3 = reverse_words_manually(&words);
    println!("{}", reverse3);

    let words = String::from("Eid Happy");
    let reverse4 = reverse_words(&words);
    println!("{}", reverse4);
}

fn reverse_word_manually(word: &str) -> String {
    let mut reverse = String::with_capacity(word.len());
    let mut chars = word.chars();

    while let Some(c) = chars.next_back() {
        reverse.push(c)
    }

    reverse
}

fn reverse_word(word: &str) -> String {
    return word.chars().rev().collect();
}

fn reverse_words_manually(words: &str) -> String {
    let mut reverse = String::with_capacity(words.len());
    let mut chars = words.split_whitespace();

    if let Some(c) = chars.next_back() {
        reverse.push_str(c);
    }

    while let Some(c) = chars.next_back() {
        reverse.push(' ');
        reverse.push_str(c);
    }
    reverse
}

fn reverse_words(words: &str) -> String {
    let mut reverse = String::with_capacity(words.len());
    let mut chars = words.split_whitespace().rev();

    if let Some(c) = chars.next() {
        reverse.push_str(c);
    }

    for c in chars {
        reverse.push(' ');
        reverse.push_str(c);
    }
    reverse
}
