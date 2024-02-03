fn main() {
    let n = 10;
    println!("{}", fibonacci_rec(n));
}

fn fibonacci_rec(n: u64) -> u64 {
    match n {
        1 | 2 => 1, // 1 | 2 is a pattern that matches either 1 or 2
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2), // _ is a catch-all value
    }
}
