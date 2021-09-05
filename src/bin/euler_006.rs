//! The sum of the squares of the first ten natural numbers is,
//!
//!     1^2 + 2^2 + 3^2 + ... + 10^2 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//!
//!     (1 + 2 + 3 + ... + 10) ^ 2 = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
//!
//!     3025 - 385 = 2640
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
//!
//! ```
//! cargo build && time ./target/debug/euler_006
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//! answer = 25164150
//!
//! real	0m0.002s
//! user	0m0.001s
//! sys	0m0.001s
//! ```

fn euler_006(n: u64) -> u64 {
    let squares = (1..=n).map(|x| x.pow(2));
    let sum_squares: u64 = squares.sum();
    // println!("sum_squares = {}", sum_squares);

    let square_sum = (1..=n).sum::<u64>().pow(2);
    // println!("square_sum = {}", square_sum);
    return square_sum - sum_squares;
}

fn main() {
    let answer = euler_006(100);
    println!("answer = {}", answer)
}

#[cfg(test)]
mod tests {
    use crate::euler_006;

    #[test]
    fn example() {
        assert_eq!(euler_006(10), 2640)
    }
}
