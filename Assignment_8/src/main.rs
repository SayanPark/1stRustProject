use std::time::{Instant};

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn fibonacci_join(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    //Solution to the assignment goes here
    let (a, b) = rayon::join(|| fib_recursive(n - 1), || fib_recursive(n - 2));
    a + b
}

fn main() {
    let start = Instant::now();
    let x = fib_recursive(47);
    let duration = start.elapsed();
    println!("Recursive fibonacci answer: {}, time taken: {:?}", x, duration);
    println!("Now run with Rayon's join.");
    let start = Instant::now();
    let x = fibonacci_join(47);
    let duration = start.elapsed();
    println!("Rayon fibonacci answer: {}, time taken: {:?}", x, duration);
}
