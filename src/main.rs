fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 2) + fib(n - 1) }
}

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }
    length
}

fn main() {
    println!("fib(20) = {}", fib(20));
    println!("collatz_length(11) = {}", collatz_length(11)); // should be 15
}
