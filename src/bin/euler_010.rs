//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
//!
//! ```
//! cargo build --release && time ./target/debug/euler_010
//!    Compiling euler_rust v0.1.0 (/Users/woodnt/Code/src/github.com/woodm1979/euler_rust)
//!     Finished release [optimized] target(s) in 0.19s
//! 142913828922
//!
//! real	0m7.273s
//! user	0m7.259s
//! sys	0m0.013s
//! ```

use ::euler_rust::prime_lib;

fn main() {
    // let prime_max: u64 = 10;
    let prime_max: u64 = 2000000;
    let ans: u64 = prime_lib::Primes::new()
        .take_while(|p| p < &prime_max)
        .sum();
    println!("{}", ans)
}
