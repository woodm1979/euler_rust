use std::{cmp::Reverse, collections::BinaryHeap};

use crate::incrementer;

#[derive(Debug)]
struct Primes {
    cursor: u64,
    sieve_pieces: BinaryHeap<Reverse<(u64, incrementer::Incrementer)>>,
}

impl Primes {
    fn new() -> Primes {
        Primes {
            cursor: 1,
            sieve_pieces: BinaryHeap::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sieve_pieces.is_empty() {
            self.cursor = 2;
            self.sieve_pieces
                .push(Reverse((2, incrementer::Incrementer::new(2))));
            return Some(self.cursor);
        }

        self.cursor += 1;

        loop {
            if let Some(Reverse(tup)) = self.sieve_pieces.peek() {
                let incr_val = tup.0;
                if incr_val > self.cursor {
                    self.sieve_pieces.push(Reverse((
                        self.cursor,
                        incrementer::Incrementer::new(self.cursor),
                    )));
                    return Some(self.cursor);
                }
            }
            if let Some(Reverse(tup)) = self.sieve_pieces.pop() {
                let incr_val = tup.0;
                if incr_val == self.cursor {
                    self.cursor += 1;
                }
                let mut incr = tup.1;
                let incr_next = incr.next().unwrap();
                self.sieve_pieces.push(Reverse((incr_next, incr)));
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
