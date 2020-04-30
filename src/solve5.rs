use crate::helpers::math::lcm;

pub fn solve5() -> u64 {

    let max = 20;
    let mut res = 2;

    for i in 2..max + 1 {
        res = lcm(res, i);
    }

    res
}