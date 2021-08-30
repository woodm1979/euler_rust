// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
//
// cargo build --release && time ./target/release/euler_003
//    Compiling euler_rust v0.1.0 (/Users/woodnt/Code/src/github.com/woodm1979/euler_rust)
//     Finished release [optimized] target(s) in 0.46s
// [71, 839, 1471, 6857]
//
// real	0m0.100s
// user	0m0.002s
// sys	0m0.001s

use std::error::Error;

use ::euler_rust::prime_lib;

fn main() -> Result<(), Box<dyn Error>> {
    // let n = 13195;
    let n = 600851475143;
    let factors = prime_lib::prime_factors(&n)?;
    println!("{:?}", factors);
    Ok(())
}
