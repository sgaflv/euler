

pub fn solve9() -> u64 {
    for a in 3u64..1000 {
        for b in 2u64..a {
            for c in 1u64..b {
                if a+b+c != 1000 {
                    continue;
                }
                if b*b+c*c == a*a {
                    return a*b*c;
                }
            }
        }
    }

    0
}