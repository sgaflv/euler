

pub fn solve6() -> u64 {

    let max = 100;
    let mut sum = 0u64;
    let mut sq = 0u64;

    for i in 1..max + 1 {
        sum += i*i;
        sq += i;
    }

    sq *= sq;

    sq - sum
}