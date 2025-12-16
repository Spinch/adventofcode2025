use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().map(|l| {
        let mut l_split = l.split(": ");
        let key = l_split.next().unwrap();
        let values = l_split.next().unwrap().split(" ").collect();
        (key, values)
    }).collect()
}

fn make_all_paths<'a>(server_rack: &HashMap<&'a str, Vec<&'a str>>, start: &'a str) ->Vec<Vec<&'a str>> {
    let mut follow_queue = VecDeque::from([(start, vec![start])]);
    let mut all_paths = Vec::new();
    while !follow_queue.is_empty() {
        let (current, path) = follow_queue.pop_front().unwrap();
        if let Some(values) = server_rack.get(&current) {
            for &follower in values {
                if path.contains(&follower) {
                    panic!("path {:?} contains {} follower", path, follower);
                }
                if follower == "out" {
                    all_paths.push(path.clone());
                }
                else {
                    let mut new_path = path.clone();
                    new_path.push(follower);
                    follow_queue.push_back((follower, new_path))
                }
            }
        }
        else { panic!("unexpected {}", current); }
    }
    all_paths
}

pub fn solve1<'a>(server_rack: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
    make_all_paths(server_rack, "you").len() as u64
}

pub fn solve2<'a>(server_rack: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
    let mut queue = VecDeque::from([("svr", [1, 0, 0, 0])]);
    let mut visited = HashMap::from([("svr", [0; 4])]);
    while !queue.is_empty() {
        let (current, mut n_paths) = queue.pop_front().unwrap();
        match current {
            "fft" => {
                n_paths[1] += n_paths[0];
                n_paths[3] += n_paths[2];
            }
            "dac" => {
                n_paths[2] += n_paths[0];
                n_paths[3] += n_paths[1];
            }
            _ => {}
        }

        let ent : &mut [u64; 4] = visited.get_mut(current).unwrap();
        for (e, p) in ent.iter_mut().zip(&n_paths) {*e += *p;}

        if current == "out" {
            continue;
        }

        for &follower in server_rack.get(current).unwrap().iter() {
            if visited.contains_key(follower) {
                if let Some(ent) = queue.iter_mut().find(|x| x.0 == follower) {
                    for (e, p) in ent.1.iter_mut().zip(&n_paths) {*e += *p;}
                }
                else {
                    queue.push_front((follower, n_paths));
                }
            }
            else {
                visited.insert(follower, [0;4]);
                queue.push_back((follower, n_paths));
            }
        }
    }
    visited["out"][3]
}

pub fn solve(input: &str) -> (u64, u64) {
    let server_rack = parse(input);
    (solve1(&server_rack), solve2(&server_rack))
}
pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve1(
                &parse(
                fs::read_to_string("test_data/day11_0.txt")
                    .unwrap()
                    .as_str())
            ), 5);
        assert_eq!(
            solve2(
                &parse(
                fs::read_to_string("test_data/day11_1.txt")
                    .unwrap()
                    .as_str())
            ), 2);
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
