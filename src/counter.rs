pub struct Counter {
    count: u64,
    step_size: u64,
}

impl Counter {
    pub fn new(step_size: u64) -> Counter {
        Counter {
            count: 0,
            step_size,
        }
    }
}

impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += self.step_size;
        Some(self.count)
    }
}

#[cfg(test)]
mod tests {
    use crate::counter;
    #[test]
    fn it_works() {
        let primes: Vec<_> = counter::Counter::new(2).take_while(|i| i < &10).collect();
        assert_eq!(primes, [2, 4, 6, 8])
    }
}
