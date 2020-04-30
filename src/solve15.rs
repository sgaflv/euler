use crate::helpers::factor::{factorial, divide_factorizations, factorization_to_number};


pub fn solve15() -> u128 {

    let a = factorial(40);
    let b = factorial(20);
    let mut c = divide_factorizations(&a, &b);
    c = divide_factorizations(&c, &b);

    factorization_to_number(&c)
}