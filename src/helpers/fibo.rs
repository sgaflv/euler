pub struct FibonacciIterator {
    pub num1: u64,
    pub num2: u64,
}

impl FibonacciIterator {
    pub fn new() -> FibonacciIterator {
        FibonacciIterator {
            num1: 0,
            num2: 1
        }
    }
}

impl Iterator for FibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {

        let next = self.num1 + self.num2;

        if next > u64::MAX / 2 {
            return None;
        }

        self.num1 = self.num2;
        self.num2 = next;

        Some(next)
    }
}