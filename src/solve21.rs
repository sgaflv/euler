
fn div_sum(a : u64) -> u64 {
    let mut sum = 0;
    for i in 1..(a/2+1) {
        if a % i == 0 {
            sum += i;
        }
    }

    sum
}

fn is_amic(a : u64) -> bool {
    let sum1 = div_sum(a);
    let sum2 = div_sum(sum1);

    sum2 == a && sum1 != a
}

pub fn solve21() -> u64 {

    let mut sum = 0u64;

    for i in 2..10000 {
        if is_amic(i) {
            println!("{} is amicable", i);
            sum += i;
        }
    }

    sum
}