use std::fs;
use utils::Field;
use crate::utils::utils;
use itertools::Itertools;

fn parse(input: &str) -> Field<bool> {
    let c = input.lines().next().unwrap().len();
    for l in input.lines() {
        assert_eq!(c, l.len());
    }
    let input_vec = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            match c {
                '.' => false,
                '@' => true,
                _ => panic!("unknown char {}", c)
            }
        })
        .collect::<Vec<_>>();
    Field::from_vec(input_vec, input.lines().count())
}

fn get_accessible_rolls(field: &Field<bool>) -> Vec<(usize, usize)> {
    (0..field.data.rows()).cartesian_product(0..field.data.cols()).filter(|c| {
        let c = (c.0, c.1);
        if field.data[c] == false {
            return false;
        }
        field.neighbors8(c).filter(|neighbor| {
            match neighbor {
                None => false,
                Some(v ) => *v.val,
            }
        }).count() <= 3
    }).collect()
}

pub fn solve(input: &str) -> (u64, u64) {

    let mut field = parse(input);

    let mut rolls_to_remove= get_accessible_rolls(&field);

    let ans1 = rolls_to_remove.len() as u64;

    let mut total = 0;
    while !rolls_to_remove.is_empty() {
        total += rolls_to_remove.len();
        for &c in rolls_to_remove.iter() {
            field.data[c] = false;
        }
        rolls_to_remove= get_accessible_rolls(&field);
    }

    let ans2 = total as u64;

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day04_0.txt")
                    .unwrap()
                    .as_str()
            ), (13, 43));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
