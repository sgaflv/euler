use crate::helpers::prime::PrimeIterator;

pub fn solve10() -> u64 {
    PrimeIterator::new().take_while(|&x| x < 2000000).sum()
}
