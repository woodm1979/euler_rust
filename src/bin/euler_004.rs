//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ```
//! cargo build --release && time ./target/release/euler_004
//!    Compiling euler_rust v0.1.0 (/Users/woodnt/Code/src/github.com/woodm1979/euler_rust)
//!     Finished release [optimized] target(s) in 0.47s
//! Found 995 * 583 = 580085
//! Found 993 * 913 = 906609
//! Max palindrome is 993 * 913 = 906609
//!
//! real	0m0.154s
//! user	0m0.003s
//! sys	0m0.002s
//! ```

fn main() {
    let mut minimum: u32 = 100;
    let mut maximum: u32 = 999;
    let mut max_known_palindrome: u32 = 1;
    for i in (minimum..=maximum).rev() {
        // println!("i = {}", i);
        for j in (minimum + 1..=i).rev() {
            // println!("i = {}, j = {}", i, j);
            let cur_mult = i * j;
            if cur_mult <= max_known_palindrome {
                break;
            }
            if is_palindromic(&cur_mult) {
                max_known_palindrome = cur_mult;
                maximum = i;
                minimum = j;
                println!("Found {} * {} = {}", i, j, cur_mult);
                break;
            }
        }
        if i * i <= max_known_palindrome {
            break;
        }
    }
    println!(
        "Max palindrome is {} * {} = {}",
        maximum, minimum, max_known_palindrome
    );
}

fn is_palindromic<T: std::fmt::Display>(item: &T) -> bool {
    let word = item.to_string();
    let drow: String = word.chars().rev().collect();
    return *word == drow;
}

// TODO: Add tests
