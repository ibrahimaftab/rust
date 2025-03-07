fn main() {
    let days = 12;
    assert!(
        (1..=12).contains(&days),
        "The number of days must be between 1 and 12."
    );

    let numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let sentenses = [
        "A Partridge in a Pear Tree.",
        "Two Turtle Doves,",
        "Three French Hens,",
        "Four Calling Birds,",
        "Five Golden Rings,",
        "Six Geese a Laying,",
        "Seven Swans a Swimming,",
        "Eight Maids a Milking,",
        "Nine Ladies Dancing,",
        "Ten Lords a Leaping,",
        "Eleven Pipers Piping,",
        "Twelve Drummers Drumming,",
    ];

    for n in 0..=days - 1 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            numbers[n]
        );

        for i in (0..=n).rev() {
            let first_word = if n != 0 && i == 0 { "and " } else { "" };
            println!("{}{}", first_word, sentenses[i])
        }

        println!()
    }
}
