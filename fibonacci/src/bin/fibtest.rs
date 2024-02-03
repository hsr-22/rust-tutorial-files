fn main() {
    let n = 10;
    println!("{}", fibonacci_nonrec(n));
}

fn fibonacci_nonrec(n: u64) -> u64 {
    match n {
        1 | 2 => 1,
        _ => {
            let mut curr = 1;
            let mut prev = 1;
            let mut sum = 0;
            for _i in 2..n {
                sum = curr + prev;
                prev = curr;
                curr = sum;
            }
            sum
        }
    }
}

#[cfg(test)]
mod fibtests {
    use super::*; // include the code from the outer scope

    #[test] // this attribute indicates that the function is a test
    fn test_basecases() {
        assert_eq!(fibonacci_nonrec(1), 1); // assert_eq! is a macro that checks if the two arguments are equal
        assert_eq!(fibonacci_nonrec(2), 1);
    }
}