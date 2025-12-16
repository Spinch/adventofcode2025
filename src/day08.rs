use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u64>> {
    let res = input.lines().map(|line| line.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    assert!(res.iter().all(|c| c.len() == 3));
    res
}

pub fn solve(input: &str, n: usize) -> (u64, u64) {

    let coordinates = parse(input);

    let mut all_pairs_dist_ind = coordinates.iter().enumerate().combinations(2).map(|c| {
        let dist = c[0].1.iter().zip(c[1].1.iter()).map(|(c1, c2)| (c1.abs_diff(*c2)).pow(2)).sum::<u64>();
        (dist, c[0].0, c[1].0)
    }).collect::<Vec<_>>();
    all_pairs_dist_ind.sort();

    let mut ans1 = 0;

    let mut indexes_to_circuits = HashMap::new();
    let mut next_available_circuit_index = 0;
    let mut circuits_total = 0;
    let mut last_pair = (&vec!(0;3), &vec!(0;3));
    for (s, (_, i1, i2)) in all_pairs_dist_ind.iter().enumerate() {
        let i1_circuit = indexes_to_circuits.get(&i1);
        let i2_circuit = indexes_to_circuits.get(&i2);
        match (i1_circuit, i2_circuit) {
            (None, None) => {
                indexes_to_circuits.insert(i1, next_available_circuit_index);
                indexes_to_circuits.insert(i2, next_available_circuit_index);
                next_available_circuit_index += 1;
                circuits_total += 1;
            },
            (None, Some(&j)) | (Some(&j), None) => {
                indexes_to_circuits.insert(i1, j);
                indexes_to_circuits.insert(i2, j);
            }
            (Some(&j1), Some(&j2)) => {
                if j1 != j2 {
                    indexes_to_circuits.values_mut().for_each(|v| { if *v == j1 { *v = j2 } });
                    circuits_total -= 1;
                }
            }
        }
        if circuits_total == 1 && indexes_to_circuits.len() == coordinates.len() {
            last_pair = (&coordinates[*i1], &coordinates[*i2]);
            break;
        }
        if s == n-1 {
            let mut circuits_total = HashMap::new();
            for circuit_ind in indexes_to_circuits.values() {
                *circuits_total.entry(circuit_ind).or_insert(0u64) += 1;
            }
            let mut circuits_total = circuits_total.values().collect::<Vec<_>>();
            let (circuits_total_3_top, _, _) = circuits_total.select_nth_unstable_by(3, |a, b| b.cmp(a));

            ans1 = circuits_total_3_top.iter().map(|x| **x).product::<u64>();
        }
    }

    let ans2 = last_pair.0[0] * last_pair.1[0];

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day08_0.txt")
                    .unwrap()
                    .as_str()
                , 10
            ), (40, 25272));
    }
    let ans = solve(input,1000);
    println!("{}, {}", ans.0, ans.1);
}
