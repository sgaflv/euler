use crate::helpers::prime::PrimeIterator;

pub fn solve7() -> u64 {
    let max = 10001;
    PrimeIterator::new().nth(max - 1).unwrap()
}