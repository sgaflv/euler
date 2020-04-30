
fn is_leap(y: usize) -> bool {
    let mut res = false;

    if y % 4 == 0 {
        res = true;
    }

    if y % 100 == 0 {
        res = false;
    }

    if y % 400 == 0 {
        res = true;
    }

    res
}

static NORMAL: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
static LEAP: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn month_days(y : usize, m: usize) -> u64 {
    let days = if is_leap(y) {
        LEAP
    } else {
        NORMAL
    };

    days[m]
}

pub fn solve19() -> u64 {
    let mut sum = 0u64;
    let mut sun = 0u64;

    for y in 1900..2001 {
        for m in 0..12 {

            sum += month_days(y, m);

            if y < 1901 {
                continue;
            }

            if sum % 7 == 6 {
                println!();
                sun += 1;
            }

        }

    }

    sun
}