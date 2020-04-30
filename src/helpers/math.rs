pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let c = a % b;
        a = b;
        b = c;
    }

    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

