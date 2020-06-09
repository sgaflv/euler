extern crate num;

use num::{BigInt, FromPrimitive};


pub fn solve25() -> u64 {

    let mut n1: BigInt = FromPrimitive::from_usize(0).unwrap();
    let mut n2: BigInt = FromPrimitive::from_usize(1).unwrap();

    for i in 1..10000 {
        let sum: BigInt = n1 + n2.clone();

        n1 = n2;
        n2 = sum;

        let s =  n1.to_str_radix(10);
        if s.len() == 1000 {
            println!("{} {}", i, s);
            return i
        }
    }

    0
}