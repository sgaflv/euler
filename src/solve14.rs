use core::cmp;

fn collatz(num: u64) -> u64 {
    let mut num = num;

    let mut cnt = 1;

    while num != 1 {

        if num % 2 == 0 {
            cnt += 1;
            num = num / 2;
        } else {
            cnt += 1;
            num = num * 3 + 1;
        }

    }

    cnt
}

pub fn solve14() -> u64 {

    let mut max = 1;
    let mut best = 0;
    for i in 1..1000000 {
        let cur = collatz(i);

        if cur > max {
            max = cur;
            best = i;
        }
        max = cmp::max(max, cur);

    }

    best
}
