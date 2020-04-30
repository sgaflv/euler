use num::BigUint;
use std::str::FromStr;

pub fn solve16() -> u64 {
    let mut number: BigUint = BigUint::from_str("1").unwrap();

    for _ in 0..1000 {
        number = number * 2u32;
    }

    let st = number.to_str_radix(10);

    let mut res = 0u64;

    for &b in st.as_bytes().iter() {
        res += (b - b'0') as u64;
    }

    res
}