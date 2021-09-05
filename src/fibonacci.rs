pub struct Fibonacci {
    penultimate: u64,
    ultimate: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            penultimate: 0,
            ultimate: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let old_first = self.penultimate;
        self.penultimate = self.ultimate;
        self.ultimate = old_first + self.ultimate;
        Some(old_first)
    }
}
