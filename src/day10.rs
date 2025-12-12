use std::fs;
use itertools::Itertools;
use regex::Regex;
use z3;

// fn parse(input: &str) -> Vec<(u64, Vec<Vec<usize>>, Vec<usize>)> {
fn parse(input: &str) -> Vec<(u64, Vec<u64>, Vec<u64>)> {
    let re = Regex::new(r"\[([.#]+)\] ((?:\(\d+(?:,\d+)*\) )+)\{(\d+(?:,\d+)*)\}").unwrap();
    input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let indicator_lights_str = caps.get(1).unwrap().as_str();
        let mut indicator_lights = 0;
        for c in indicator_lights_str.chars().rev() {
            indicator_lights = indicator_lights << 1;
            match c {
                '#' => indicator_lights += 1,
                '.' => (),
                x => panic!("unknown char: {}", x),
            }
        }
        let buttons_str = caps.get(2).unwrap().as_str();
        let buttons = buttons_str.strip_suffix(" ").unwrap().split(" ").map(|button| {
            let mut button_bin = 0;
            for b in button.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split(",") {
                let bi:usize = b.parse().unwrap();
                button_bin |= 1 << bi;
            }
            button_bin
        }).collect();

        let joltage = caps.get(3).unwrap().as_str().split(",").map(|j| {
            j.parse().unwrap()
        }).collect();

        (indicator_lights, buttons, joltage)
    }).collect()
}

fn brut_force_buttons(indicator_lights: u64, buttons: &Vec<u64>) -> Option<u64> {
    for i in 1..=buttons.len() {
        for comb in buttons.iter().combinations(i) {
            let mut buttons_comb_press = 0;
            for c in comb {
                buttons_comb_press ^= c;
            }
            if buttons_comb_press == indicator_lights {
                return Some(i as u64);
            }
        }
    }
    None
}

fn solve_joltage_with_z3(buttons: &Vec<u64>, joltage: &Vec<u64>) -> u64 {
    let solver = z3::Solver::new();
    let nb = buttons.len();
    let nj = joltage.len();
    let xs = (0..nb).map(|i| {
        z3::ast::Int::new_const(format!("x{}", i))
    }).collect::<Vec<_>>();
    let mut adds = vec![z3::ast::Int::from_u64(0); nj];
    for (bi, b) in buttons.iter().enumerate() {
        let mut b = b.clone();
        for ji in 0..nj {
            if b & 1 == 1 {
                adds[ji] = z3::ast::Int::add(&[&adds[ji], &xs[bi]])
            }
            b = b >> 1;
        }
    }
    for (i, add) in adds.iter().enumerate() {
        solver.assert(add.eq(joltage[i]));
    }
    for x in xs.iter() {
        solver.assert(x.ge(0));
    }
    solver.into_solutions(&xs, true).map(|s| s.iter().map(|i| i.as_u64().unwrap()).sum()).min().unwrap()
}

pub fn solve(input: &str) -> (u64, u64) {

    let machines = parse(input);
    let ans1 = machines.iter().map(|machine| {
        let (indicator_lights, buttons, _) = machine;
        brut_force_buttons(*indicator_lights, buttons).unwrap()
    }).sum();
    // let ans1 = 0;
    let ans2 = machines.iter().map(|machine| {
        let (_, buttons, joltage) = machine;
        solve_joltage_with_z3(buttons, joltage)
    }).sum();

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day10_0.txt")
                    .unwrap()
                    .as_str()
            ), (7, 33));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
