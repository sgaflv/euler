use num::{BigUint};

pub fn solve20() -> u64 {

    let mut a: BigUint = BigUint::from(1u64);

    for i in 1u64..100 {
        a = a * i;
    }

    let mut sum = 0u64;

    a.to_str_radix(10).as_bytes().iter().for_each(|&b| {
        sum += (b - b'0') as u64;
    });

    sum
}