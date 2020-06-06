const MAX: u64 = 28123u64;

static mut DS: [u64; MAX as usize] = [0; MAX as usize];

fn d_sum(a : u64) -> u64 {
    unsafe {
        if DS[a as usize] != 0 {
            return DS[a as usize];
        }
    }

    let mut sum = 0;
    for i in 1..(a/2+1) {
        if a % i == 0 {
            sum += i;
        }
    }

    unsafe {
        DS[a as usize] = sum;
    }

    sum
}

pub fn solve23() -> u64 {

    let mut sum = 0;

    for x in 1 .. MAX + 1 {
        let mut found = false;

        for i in 2 .. x / 2 + 1 {
            let i2 = x - i;

            if d_sum(i) > i && d_sum(i2) > i2 {
                found = true;
                break;
            }
        }

        if !found {
            sum += x;
            println!("{} is non-abundant", x);
        }
    }

    sum
}