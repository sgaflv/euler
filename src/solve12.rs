use crate::helpers::prime::factor;

fn count_dividors(num: u64) -> u64 {

    let factor = factor(num);

    let mut cnt = 1;
    for (_, p) in factor {
        cnt *= p + 1;
    }

    cnt as u64
}

pub fn solve12() -> u64 {
    let max = 500;

    let mut cur = 0u64;

    for i in 1..100000u64 {
        cur = i * (i + 1) / 2;

        let cnt = count_dividors(cur);

        if cnt > max {
            return cur;
        }
    }

    cur
}