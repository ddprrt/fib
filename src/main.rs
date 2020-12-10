fn fib(i: u64) -> u64 {
    match i {
        0 | 1 | 2 => 1,
        _ => fib(i - 2) + fib(i - 1),
    }
}

fn main() {
    for i in 1..=20 {
        println!("Fibonacci {:3}: {}", i, fib(i));
    }
}

#[test]
fn fib_20() {
    assert_eq!(fib(20), 6765);
}
