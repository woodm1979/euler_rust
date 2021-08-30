use std::{cmp::Reverse, collections::BinaryHeap};

use crate::incrementer;

/// An iterator that will return prime numbers until you run out of memory.
///
/// This prime-number generator is based on the Sieve of Eratosthenes.
/// I've chosen to keep a heap of iterators instead of a giant array of candidate numbers.
/// I believe this is more memory efficient, and was DEFINITELY more fun to write.
///
/// # Examples
///
/// ```
/// let primes: Vec<_> = euler_rust::prime_lib::Primes::new().take_while(|i| i < &20).collect();
/// assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19])
/// ```
#[derive(Debug)]
pub struct Primes {
    cursor: u64,
    sieve_pieces: BinaryHeap<Reverse<incrementer::Incrementer>>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            cursor: 1,
            sieve_pieces: BinaryHeap::new(),
        }
    }

    fn push_new_incrementer(&mut self) {
        self.sieve_pieces
            .push(Reverse(incrementer::Incrementer::new(self.cursor)))
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sieve_pieces.is_empty() {
            // We haven't really gotten started yet.  2 is the known first prime.
            self.cursor = 2;
            self.push_new_incrementer();
            return Some(self.cursor);
        }

        // Whatever the previous first prime was, we've already returned it.
        self.cursor += 1;

        loop {
            if let Some(Reverse(mut incr)) = self.sieve_pieces.pop() {
                if incr.current_val > self.cursor {
                    // We found a prime!  It's self.cursor!
                    self.push_new_incrementer();
                    // Don't forget to put our currently popped incr back.
                    self.sieve_pieces.push(Reverse(incr));
                    return Some(self.cursor);
                }
                if incr.current_val == self.cursor {
                    // This value is not prime as it's been eliminated by one of our incrementers.
                    self.cursor += 1;
                }

                // This incrementer needs to be updated and put back in self.sieve_pieces.
                incr.next();
                self.sieve_pieces.push(Reverse(incr));
            }
        }
    }
}

/// Get all prime factors of a number n.
/// # Examples
///
/// ```
/// let factors = euler_rust::prime_lib::prime_factors(&12).unwrap();
/// assert_eq!(factors, [2, 2, 3])  // Because 2 * 2 * 3 = 12
/// ```
pub fn prime_factors(n: &u64) -> Result<Vec<u64>, &'static str> {
    if *n < 2 {
        return Err("Cannot determine prime factors of number less than 2");
    }
    let mut factored = n.clone();
    let mut factors = Vec::new();
    let primes = Primes::new();

    for prime in primes {
        while factored % prime == 0 {
            factored /= prime;
            factors.push(prime);
        }
        if prime > factored {
            break;
        }
    }
    return Ok(factors);
}

#[cfg(test)]
mod tests {
    mod primes_tests {
        use crate::prime_lib::Primes;

        #[test]
        fn it_works() {
            let max: u64 = 20;
            let primes: Vec<_> = Primes::new().take_while(|i| i < &max).collect();
            assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19])
        }
    }

    mod prime_factors_tests {
        use crate::prime_lib::prime_factors;

        #[test]
        fn errors_if_less_than_2() -> Result<(), String> {
            assert_eq!(
                prime_factors(&1),
                Err("Cannot determine prime factors of number less than 2")
            );
            Ok(())
        }

        #[test]
        fn n_equals_prime() -> Result<(), String> {
            assert_eq!(prime_factors(&5)?, [5]);
            Ok(())
        }

        #[test]
        fn n_has_multiple_factors() -> Result<(), String> {
            assert_eq!(prime_factors(&10)?, [2, 5]);
            Ok(())
        }

        #[test]
        fn n_has_duplicate_factors() -> Result<(), String> {
            assert_eq!(prime_factors(&36)?, [2, 2, 3, 3]);
            Ok(())
        }

        #[test]
        fn n_is_semi_big() -> Result<(), String> {
            assert_eq!(prime_factors(&13195)?, [5, 7, 13, 29]);
            Ok(())
        }
    }
}
