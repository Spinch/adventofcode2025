use std::fs;

fn parse(input: &str) -> Vec<(i32, u32)> {
    input.lines().map(|line| {
        let mut chars = line.chars();
        let direction = match chars.next().unwrap() {
            'L' => -1,
            'R' => 1,
            c => panic!("invalid direction symbol: {}", c),
        };
        let nums = chars.collect::<String>().parse::<u32>().unwrap();
        (direction, nums)
    }).collect()
}

pub fn solve(input: &str) -> (u64, u64) {
    let code = parse(input);

    let mut base = 50;
    let ans1 = code.iter().filter(|(direction, nums)| {
        base = (base + direction * (*nums as i32)) % 100;
        base == 0
    }).count() as u64;

    base= 50;
    let ans2 = code.iter().map(|(direction, nums)| {
        if base*direction < 0 {
            base += direction*100;
        };
        let new_base = base + direction * (*nums as i32);
        base = new_base % 100;
        (new_base / 100).abs()
    }).sum::<i32>() as u64;

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day01_0.txt")
                    .unwrap()
                    .as_str()
            ), (3, 6));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
