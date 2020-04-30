use crate::helpers::fibo::FibonacciIterator;

pub fn solve2() -> u64 {
    FibonacciIterator::new().
        take_while(|&i| i <= 4000000).
        filter(|&i| i % 2 == 0).sum()
}
