use std::io;

fn main() {
    println!("Enter the number");
    let mut num: String = String::new();

    io::stdin().read_line(&mut num).expect("Failed to read");
    let num: usize = num
        .trim()
        .parse()
        .expect("Please enter valid positive number");
    let result = fib(num);
    println!("fib of {num} is {result}");
}

fn fib(num: usize) -> usize {
    if num <= 1 {
        num
    } else {
        fib(num - 1) + fib(num - 2)
    }
}
