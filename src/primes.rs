use std::{cmp::Reverse, collections::BinaryHeap};

use crate::incrementer;

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
                    // Don't forget to put our current-popped incr back.
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

#[cfg(test)]
mod tests {
    use crate::primes::Primes;
    #[test]
    fn it_works() {
        let max: u64 = 20;
        let primes_below_max: Vec<_> = Primes::new().take_while(|i| i < &max).collect();
        assert_eq!(primes_below_max, [2, 3, 5, 7, 11, 13, 17, 19])
    }
}
