//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
//!
//! ```
//! cargo build && time ./target/debug/euler_007
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//! 104743
//!
//! real	0m0.291s
//! user	0m0.289s
//! sys	0m0.001s
//! ```

use ::euler_rust::prime_lib;

fn main() {
    let x = prime_lib::Primes::new().nth(10000).unwrap();
    println!("{}", x);
}
