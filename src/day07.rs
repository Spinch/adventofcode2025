use std::collections::{HashMap, HashSet};
use std::fs;

fn parse(input: &str) -> (u32, Vec<HashSet<u32>>) {
    let start_ind = input.lines().nth(0).unwrap().chars().position(|c| c == 'S').unwrap() as u32;
    let splitters_ind = input.lines().skip(0).map(|l| {
        l.chars().enumerate().filter_map(|(i, c)| {
            match c == '^' {
                true => Some(i as u32),
                false => None,
            }
        }).collect()
    }).collect();
    (start_ind, splitters_ind)
}

pub fn solve(input: &str) -> (u64, u64) {

    let (start, splitters) = parse(input);

    let mut splits_number = 0;
    let mut beams = HashMap::new();
    beams.insert(start, 1);
    for split_line in splitters.iter() {
        let mut new_beams = HashMap::new();
        for (&beam_ind, &beam_cost) in beams.iter() {
            match split_line.contains(&beam_ind) {
                false => {*new_beams.entry(beam_ind).or_insert(0) += beam_cost;},
                true => {
                    splits_number += 1;
                    *new_beams.entry(beam_ind-1).or_insert(0) += beam_cost;
                    *new_beams.entry(beam_ind+1).or_insert(0) += beam_cost;
                },
            }
        }
        beams = new_beams;
    }

    let ans1 = splits_number;
    let ans2 = beams.values().sum::<u64>();

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day07_0.txt")
                    .unwrap()
                    .as_str()
            ), (21, 40));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
