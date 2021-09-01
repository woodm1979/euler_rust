//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ```
//! cargo build --release && time ./target/release/euler_004
//!    Compiling euler_rust v0.1.0 (/Users/woodnt/Code/src/github.com/woodm1979/euler_rust)
//!     Finished release [optimized] target(s) in 0.29s
//! Found 913 * 993 = 906609
//! Max palindrome is 913 * 993 = 906609 found after 2169 checks
//!
//! real	0m0.113s
//! user	0m0.001s
//! sys	0m0.001s
//! ```

fn main() {
    search_diagonally();
    // println!("---------------------");
    // search_vertically();
}

fn search_diagonally() {
    let mut checked: u32 = 0;
    let mut minimum: u32 = 100;
    let mut maximum: u32 = 999;
    let mut max_known_palindrome: u32 = 1;
    for diff in 0..maximum - minimum {
        let mut i = maximum - diff;
        let mut j = maximum.clone();

        // println!("i = {}", i);
        while j >= i {
            // println!("i = {}, j = {}", i, j);
            let cur_mult = i * j;
            if cur_mult <= max_known_palindrome {
                break;
            }
            checked += 1;
            if is_palindromic(&cur_mult) {
                max_known_palindrome = cur_mult;
                maximum = i;
                minimum = j;
                println!("Found {} * {} = {}", i, j, cur_mult);
                break;
            }
            j -= 1;
            i += 1;
        }
        if i * i <= max_known_palindrome {
            break;
        }
    }
    println!(
        "Max palindrome is {} * {} = {} found after {} checks",
        maximum, minimum, max_known_palindrome, checked,
    );
}

// fn search_vertically() {
//     let mut checked: u32 = 0;
//     let mut minimum: u32 = 100;
//     let mut maximum: u32 = 999;
//     let mut max_known_palindrome: u32 = 1;
//     for i in (minimum..=maximum).rev() {
//         // println!("i = {}", i);
//         for j in (minimum + 1..=i).rev() {
//             // println!("i = {}, j = {}", i, j);
//             let cur_mult = i * j;
//             if cur_mult <= max_known_palindrome {
//                 break;
//             }
//             checked += 1;
//             if is_palindromic(&cur_mult) {
//                 max_known_palindrome = cur_mult;
//                 maximum = i;
//                 minimum = j;
//                 println!("Found {} * {} = {}", i, j, cur_mult);
//                 break;
//             }
//         }
//         if i * i <= max_known_palindrome {
//             break;
//         }
//     }
//     println!(
//         "Max palindrome is {} * {} = {} found after {} checks",
//         maximum, minimum, max_known_palindrome, checked,
//     );
// }

fn is_palindromic<T: std::fmt::Display>(item: &T) -> bool {
    let word = item.to_string();
    let drow: String = word.chars().rev().collect();
    return *word == drow;
}

// TODO: Add tests
