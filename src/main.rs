fn fib(i: u64) -> u64 {
    match i {
        0 | 1 | 2 => 1,
        _ => fib(i - 2) + fib(i - 1),
    }
}

fn main() {
    println!("{}", fib(20));
}
