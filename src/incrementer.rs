#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Incrementer {
    count: u64,
    step_size: u64,
}

impl Incrementer {
    pub fn new(step_size: u64) -> Incrementer {
        Incrementer {
            count: 0,
            step_size,
        }
    }
}

impl Iterator for Incrementer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += self.step_size;
        Some(self.count)
    }
}

#[cfg(test)]
mod tests {
    use crate::incrementer;
    #[test]
    fn it_works() {
        let evens_below_10: Vec<_> = incrementer::Incrementer::new(2)
            .take_while(|i| i < &10)
            .collect();
        assert_eq!(evens_below_10, [2, 4, 6, 8])
    }
}
