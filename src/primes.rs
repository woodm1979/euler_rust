use std::{cmp::Reverse, collections::BinaryHeap};

use crate::incrementer;

struct Primes {
    cursor: u64,
    // sieve_pieces: BinaryHeap<Tuple<u64, incrementer::Incrementer>>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            cursor: 1,
            // sieve_pieces: BinaryHeap::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < 2 {
            self.cursor += 1;
            return Some(self.cursor);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::primes::Primes;
    #[test]
    fn it_works() {
        let max: u64 = 3;
        let primes_below_max: Vec<_> = Primes::new().take_while(|i| i < &max).collect();
        assert_eq!(primes_below_max, [2])
    }
}
