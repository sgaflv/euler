use core::cmp;

static NUMBERS_STR: &str = "\
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";


type Numbers = Vec<Vec<u64>>;

pub struct Solver {
    size: usize,
    numbers: Numbers,
    best: Numbers,
}

fn init_numbers(size: usize) -> Numbers {

    let mut numbers: Numbers = Vec::new();

    for _ in 0..size {
        numbers.push(vec![0u64; size]);
    }

    numbers
}

fn get_numbers(size: usize, numbers_str: &str) -> Numbers {

    let mut numbers: Numbers = init_numbers(size);

    for (y, line) in numbers_str.lines().enumerate() {
        for (x, item) in line.split_whitespace().enumerate() {
            numbers[y][x] = item.parse().unwrap();
        }
    }

    numbers
}

impl Solver {
    pub fn new(size: usize, numbers_str: &str) -> Solver {
        Solver {
            size,
            numbers: get_numbers(size, numbers_str),
            best: init_numbers(size),
        }
    }

    pub fn solve(&mut self, x: usize, y: usize) -> u64 {
        if x > y {
            return 0;
        }

        if x >= self.size || y >= self.size {
            return 0;
        }

        if self.best[y][x] == 0 {
            self.best[y][x] = self.numbers[y][x] + cmp::max(self.solve(x, y+1), self.solve(x+1,y+1));
        }

        self.best[y][x]
    }
}

pub fn solve18() -> u64 {

    let mut solver = Solver::new(15, NUMBERS_STR);
    let res = solver.solve(0,0);

    res
}