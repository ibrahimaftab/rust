fn main() {
    const ITEMS: [isize; 5] = [1, 2, 3, 4, 5];

    println!("Please enter the index of the array you want to get");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number");

    println!("The value of the item is {}", ITEMS[index]);
}
