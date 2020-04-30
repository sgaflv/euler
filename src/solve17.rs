

static DIG_TEEN:[&str; 21] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"];

static TENTH: [&str; 10] = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

static HUNDRED: &str = "hundred";

static THOUSAND: &str = "thousand";

static AND: &str = "and";


fn to_words(num: usize) -> Vec<&'static str> {
    let mut num = num;
    let mut result: Vec<&str> = Vec::new();

    if num == 1000 {
        result.push(DIG_TEEN[num / 1000]);
        result.push(THOUSAND);
    }

    if 99 < num && num < 1000 {
        result.push(DIG_TEEN[num / 100]);
        result.push(HUNDRED);
        num = num % 100;
        if num > 0 {
            result.push(AND);
        }
    }

    if 19 < num && num < 100 {
        result.push(TENTH[num / 10]);
        num = num % 10
    }

    if num > 0 && num < 20 {
        result.push(DIG_TEEN[num]);
    }

    result
}

pub fn solve17() -> u64 {

    let mut sum = 0u64;
    for i in 1..1001 {
        println!("{} {:?}", i, to_words(i));
        to_words(i).iter().for_each(|&s| {
            sum += s.len() as u64;
        })
    }

    sum
}