use indexmap::IndexMap;
use std::fs;

fn main() {
    run_day(None, None);
}

fn run_day(day: Option<usize>, input_file: Option<&str>) {
    let mut solutions: IndexMap<usize, &dyn Fn(&str, bool) -> ()> = IndexMap::new();
    solutions.insert(1, &ac2025::day01::run);
    solutions.insert(2, &ac2025::day02::run);
    solutions.insert(3, &ac2025::day03::run);
    solutions.insert(4, &ac2025::day04::run);
    solutions.insert(5, &ac2025::day05::run);
    solutions.insert(6, &ac2025::day06::run);
    solutions.insert(7, &ac2025::day07::run);
    solutions.insert(8, &ac2025::day08::run);
    solutions.insert(9, &ac2025::day09::run);
    solutions.insert(10, &ac2025::day10::run);
    solutions.insert(11, &ac2025::day11::run);
    solutions.insert(12, &ac2025::day12::run);

    let day = match day {
        Some(x) => x,
        None => *solutions.keys().last().unwrap(),
    };

    let file_name = match input_file {
        Some(x) => x.to_string(),
        None => format!("data/day{:02}.txt", day),
    };
    let data = fs::read_to_string(file_name).unwrap();
    solutions.get(&day).unwrap()(&data, true);
}
