pub fn is_palindrome(num: u64) -> bool {
    let a = format!("{}", num);
    let b = a.as_bytes();
    let len = b.len();

    for i in 0..len / 2
    {
        if b[i] != b[len-1-i] {
            return false;
        }
    }

    true
}


pub fn solve4() -> u64 {

    let mut max = 0;
    for x in 901..999 {
        for y in 901..999 {

            if is_palindrome(x * y) {
               if max < x * y {
                   max = x * y;
               }
            }
        }
    }

    max
}