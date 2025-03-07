fn main() {
    fibonacci(20);
    let items = fibonacci_optimized(20);
    println!("The 20th fibonacci number is {:?} \n", items);
}

fn fibonacci(n: usize) {
    let mut j = 0;
    let mut items = Vec::new();

    loop {
        let k = match j {
            0 => 0,
            1 => 1,
            num => items[num - 1] + items[num - 2],
        };

        items.push(k);
        j += 1;

        if j == n {
            break;
        }
    }

    println!("The {}th fibonacci number is {:?} \n", n, items);
}

fn fibonacci_optimized(n: usize) -> Vec<usize> {
    let mut items = Vec::new();

    for j in 0..n {
        let k = match j {
            0 => 0,
            1 => 1,
            num => items[num - 1] + items[num - 2],
        };

        items.push(k);
    }

    items
}
