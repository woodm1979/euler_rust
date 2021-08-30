//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! ```
//! cargo build --release && time ./target/release/euler_001
//! Finished release \[optimized] target(s) in 0.00s
//! Sum: 233168
//!
//! real	0m0.002s
//! user	0m0.001s
//! sys	0m0.001s
//! ```
use std::collections::HashSet;

fn get_multiples(multiplier: usize, max: &usize) -> HashSet<usize> {
    return (multiplier..*max).step_by(multiplier).collect();
}

fn euler_001(max: usize) {
    let by_3 = get_multiples(3, &max);
    let by_5 = get_multiples(5, &max);
    let by_3_or_5 = by_3.union(&by_5);

    // let mut by_3_or_5: Vec<_> = by_3_or_5.into_iter().collect();
    // by_3_or_5.sort();
    // for i in &by_3_or_5 {
    //     println!("{}", i)
    // }

    let sum: usize = by_3_or_5.into_iter().sum();
    println!("Sum: {}", sum)
}

fn main() {
    euler_001(1000)
}
