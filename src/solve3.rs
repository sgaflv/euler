

pub fn solve3() -> u64 {
    let num = 600851475143u64;

    let stop = (num as f64).sqrt() as u64;
    let mut skipped = 2;
    let mut found = 0;
    let mut i = 3;

    while i <= stop {
        if num % i == 0 && skipped % i != 0 {
            found = i;
            skipped = skipped * i;
        }

        i += 2;
    }

    found
}