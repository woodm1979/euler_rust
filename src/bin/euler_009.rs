//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//! a^2 + b^2 = c^2
//!
//! For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product a*b*c.
//!
//! ```
//! cargo build --release && time ./target/debug/euler_009
//!     Finished release [optimized] target(s) in 0.00s
//! 200 < 375 < 425
//! 200^2 + 375^2 = 40000 + 140625 = 180625 = 425^2
//! 200 * 375 * 425 = 31875000
//!
//! real	0m0.005s
//! user	0m0.003s
//! sys	0m0.001s
//! ```

fn main() {
    let max_sum: u64 = 1000;
    // for a in 1..=max_sum {
    for a in 1..=(max_sum / 3) {
        let a_squared = a * a;
        for b in (a + 1)..=max_sum {
            // we're maintaining a < b
            let c = max_sum - a - b;
            if c <= b {
                // We must maintain a < b < c
                break;
            }
            if a_squared + b * b == c * c {
                println!("{} < {} < {}", a, b, c);
                println!(
                    "{}^2 + {}^2 = {} + {} = {} = {}^2",
                    a,
                    b,
                    a_squared,
                    b * b,
                    c * c,
                    c
                );
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
            }
        }
    }
}
