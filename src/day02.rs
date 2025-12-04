use std::fs;

fn parse(input: &str) -> Vec<(u64, u64)> {
    input.split(',').map(|pair| {
        let mut pair_iter = pair.split('-');
        let first = pair_iter.next().unwrap().parse::<u64>().unwrap();
        let second = pair_iter.next().unwrap().parse::<u64>().unwrap();
        assert!(pair_iter.next().is_none());
        (first, second)
    }).collect()
}

fn is_sily(x: u64) -> bool {
    'outer: for i in 1..=(x.checked_ilog10().unwrap_or(0) + 1)/2 {
        let mut nx = x / 10u64.pow(i);
        let seq = x - nx * 10u64.pow(i);
        if seq < 10u64.pow(i-1) {
            continue 'outer;
        }
        while nx > 0 {
            let rest = nx / 10u64.pow(i);
            let pos = nx - rest * 10u64.pow(i);
            if pos != seq {
                continue 'outer;
            }
            nx = rest;
        }
        return true
    }
    false
}

fn solve_bruteforce(input: &Vec<(u64, u64)>) -> u64 {
    let mut s = 0;
    for (low, top) in input {
        for x in *low..=*top {
            if is_sily(x) {
                s+= x;
            }
        }
    }
    s
}

pub fn solve(input: &str) -> (u64, u64) {

    let pairs = parse(input);

    let ans1 = pairs.iter().map( |(low, top)| {
        let low_len = low.checked_ilog10().unwrap_or(0) + 1;
        let next_possible_low_half = if low_len % 2 == 1 {
            10u64.pow(low_len / 2)
        }
        else {
            let first_half = low / 10u64.pow(low_len/2);
            let second_half = low - first_half * 10u64.pow(low_len/2);
            if dbg!(first_half) >= dbg!(second_half) {
                first_half
            }
            else {
                first_half+1
            }
        };
        let top_len = top.checked_ilog10().unwrap_or(0) + 1;
        let prev_possible_top_half = if top_len % 2 == 1 {
            10u64.pow(top_len / 2) - 1
        }
        else {
            let first_half = top / 10u64.pow(top_len/2);
            let second_half = top - first_half * 10u64.pow(top_len/2);
            if first_half <= second_half {
                first_half
            }
            else {
                first_half-1
            }
        };
        (next_possible_low_half..=prev_possible_top_half).map(|x| x * 10u64.pow(x.checked_ilog10().unwrap_or(0)+1) + x).sum::<u64>()
    }).sum();


    let ans2 = solve_bruteforce(&pairs);

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day02_0.txt")
                    .unwrap()
                    .as_str()
            ), (1227775554, 4174379265));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
