extern crate num;

use num::{BigInt, FromPrimitive, One};


pub fn solve25() -> u64 {

    let mut n1: BigInt = FromPrimitive::from_usize(0).unwrap();
    let mut n2: BigInt = FromPrimitive::from_usize(1).unwrap();

    for i in 1..10000 {
        let sum: BigInt = n1.clone() + n2.clone();

        n1 = n2.clone();
        n2 = sum.clone();

        let s =  n1.clone().to_str_radix(10);
        if s.len() == 1000 {
            println!("{} {}", i, s);
            return i
        }
    }

    0
}