use std::fs;

fn parse(input: &str) -> (Vec<(u64,u64)>, Vec<u64>) {
    let mut parts = input.split("\n\n");
    let ranges = parts.next().unwrap().lines().map(|l| {
        let mut l_split = l.split("-");
        let first = l_split.next().unwrap().parse::<u64>().unwrap();
        let second = l_split.next().unwrap().parse::<u64>().unwrap();
        assert!(l_split.next().is_none());
        (first, second)
    }).collect::<Vec<_>>();

    let numbers = parts.next().unwrap().lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<_>>();
    (ranges, numbers)
}

fn collapse_ranges(sorted_ranges: &Vec<(u64,u64)>) -> Vec<(u64,u64)> {
    let mut new_ranges = vec![sorted_ranges[0]];
    for range in &sorted_ranges[1..] {
        if range.0 <= new_ranges.last().unwrap().1 {
            new_ranges.last_mut().unwrap().1 = range.1.max(new_ranges.last().unwrap().1);
        }
        else {
            new_ranges.push(*range);
        }
    }
    return new_ranges;
}

pub fn solve(input: &str) -> (u64, u64) {

    let (mut ranges, numbers) = parse(input);
    ranges.sort();
    let ranges = collapse_ranges(&ranges);

    let ans1 = numbers.iter().filter(|&&n| {
        for range in &ranges {
            if n <= range.1 {
                return n >= range.0;
            }
        }
        return false
    }).count() as u64;
    let ans2 = ranges.iter().map(|r| r.1 - r.0 + 1).sum::<u64>();

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day05_0.txt")
                    .unwrap()
                    .as_str()
            ), (3, 14));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
