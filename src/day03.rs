use std::fs;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

fn get_max_joltage(bank: &[u32], n: usize) -> u64 {
    let mut res = 0;
    let mut first_i = 0;
    for i in 0..n {
        let (max_i, max_v) = bank[first_i..bank.len()-n+1+i].iter().enumerate().rev().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        res = res*10 + (*max_v as u64);
        first_i = first_i+max_i+1;
    }
    res
}

pub fn solve(input: &str) -> (u64, u64) {

    let banks = parse(input);

    let ans1 = banks.iter().map(|bank| get_max_joltage(bank, 2)).sum();
    let ans2 = banks.iter().map(|bank| get_max_joltage(bank, 12)).sum();

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day03_0.txt")
                    .unwrap()
                    .as_str()
            ), (357, 3121910778619));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
