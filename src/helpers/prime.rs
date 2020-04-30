
pub fn is_prime(num: u64) -> bool {
    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let mut i = 3;
    let max = (num as f64).sqrt() as u64;
    while i <= max {
        if num % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

pub type Factorization = Vec<(u64, i64)>;

pub fn factor(num: u64) -> Factorization {
    let mut result: Factorization = Vec::new();

    let mut num = num;

    let mut cnt = 0;
    while num % 2 == 0 {
        cnt += 1;
        num /= 2;
    }
    if cnt > 0 {
        result.push((2, cnt));
    }

    let mut i = 3;
    while i * i <= num {

        let mut cnt = 0;
        while num % i == 0 {
            cnt += 1;
            num /= i;
        }
        if cnt > 0 {
            result.push((i, cnt));
        }

        i += 2;
    }

    if num > 1 {
        result.push((num, 1));
    }

    result
}

pub struct PrimeIterator {
    last_prime: u64
}

impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        PrimeIterator {
            last_prime: 0
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

        if self.last_prime < 2 {
            self.last_prime = 2;
            return Some(2)
        }

        if self.last_prime < 3 {
            self.last_prime = 3;
            return Some(3)
        }

        let mut next_prime = self.last_prime + 2;
        while !(is_prime(next_prime)) {
            next_prime += 2;
        }

        self.last_prime = next_prime;

        Some(self.last_prime)
    }
}