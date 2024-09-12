use std::io;

fn main() {
    println!("Enter the number to find in the fibonacci sequence");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let number: u32 = input.trim().parse().unwrap_or_else(|_| 0);

    let fib = fibonacci_number(number);

    println!("Answer: {fib}");
}

fn fibonacci_number(number: u32) -> u32 {
    if number == 0 {
        return 0
    }

    let mut fib = 1;
    let mut fib_prev = 0;

    for _ in 1..number {
        let tmp = fib;
        fib += fib_prev;
        fib_prev = tmp;
    }

    fib
}