
pub fn solve1() -> i32 {
    (3..1000).into_iter().filter(|&x| { x % 3 == 0 || x % 5 == 0 }).sum()
}
