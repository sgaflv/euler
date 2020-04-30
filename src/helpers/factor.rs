use crate::helpers::prime::{Factorization, factor};
use core::cmp;

pub fn multiply_factorizations(f1: &Factorization, f2: &Factorization) -> Factorization {
    let mut result: Factorization = Vec::new();

    let mut i1 = 0usize;
    let mut i2 = 0usize;

    while i1 < f1.len() || i2 < f2.len() {

        let fe1 = f1.get(i1);

        let fe2 = f2.get(i2);

        let mut min = u64::max_value();
        let mut rv = 0i64;

        if let Some(&(fk, _)) = fe1 {
            min = fk
        }

        if let Some(&(fk, _)) = fe2 {
            min = cmp::min(min, fk);
        }

        if let Some(&(fk, fv)) = fe1 {
            if min == fk {
                rv += fv;
                i1 += 1;
            }
        }

        if let Some(&(fk, fv)) = fe2 {
            if min == fk {
                rv += fv;
                i2 += 1;
            }
        }

        result.push((min, rv));
    }

    result
}

pub fn factorization_to_number(f: &Factorization) -> u128 {
    let mut result = 1u128;

    for &(k, mut v) in f {

        while v > 0 {
            result *= k as u128;
            v -= 1;
        }
    }

    result
}

pub fn divide_factorizations(f1: &Factorization, f2: &Factorization) -> Factorization {
    let mut result: Factorization = Vec::new();

    if f2.len() == 0 {
        panic!("Cannot divide by empty factorization!");
    }

    let mut i1 = 0usize;
    let mut i2 = 0usize;

    while i1 < f1.len() || i2 < f2.len() {

        let fe1 = f1.get(i1);

        let fe2 = f2.get(i2);

        let mut min = u64::max_value();
        let mut rv = 0;

        if let Some(&(fk, _)) = fe1 {
            min = fk
        }

        if let Some(&(fk, _)) = fe2 {
            min = cmp::min(min, fk);
        }

        if let Some(&(fk, fv)) = fe1 {
            if min == fk {
                rv += fv;
                i1 += 1;
            }
        }

        if let Some(&(fk, fv)) = fe2 {
            if min == fk {
                rv -= fv;
                i2 += 1;
            }
        }

        if rv > 0 {
            result.push((min, rv));
        }

    }

    result
}

pub fn factorial(num: u64) -> Factorization {
    let mut result: Factorization = Vec::new();

    for i in 2u64..num+1 {
        let f = factor(i);

        result = multiply_factorizations(&result, &f);
    }

    result
}