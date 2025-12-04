use std::fs;

pub fn solve(input: &str) -> (u64, u64) {

    let ans1 = 0;
    let ans2 = 0;

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day00_0.txt")
                    .unwrap()
                    .as_str()
            ), (0, 0));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
