

fn perm(total_col: usize, col: usize, numbers: &mut [usize; 10], taken: &mut [bool; 10]) -> u64  {
    static mut COUNTER: i32 = 0i32;

    if col == total_col {
        unsafe {
            COUNTER += 1;

            if COUNTER == 1000000 {

                let mut result = 0u64;

                for i in 0usize..total_col {
                    result *= 10;
                    result += numbers[i] as u64;
                }

                return result;
            }

        }

        return 0u64;
    }

    for i in 0..total_col {
        if taken[i] {
            continue;
        }

        numbers[col]=i;
        taken[i] = true;

        let result = perm(total_col, col + 1, numbers, taken);

        if result > 0 {
            return result;
        }

        taken[i] = false;
    }

    0u64
}

pub fn solve24() -> u64 {

    let mut numbers = [0usize; 10];
    let mut taken = [false; 10];

    perm(10, 0, &mut numbers, &mut taken)
}