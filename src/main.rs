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
    // solutions.insert(8, &ac2025::day08::run);
    // solutions.insert(9, &ac2025::day09::run);
    // solutions.insert(10, &ac2025::day10::run);
    // solutions.insert(11, &ac2025::day11::run);
    // solutions.insert(12, &ac2025::day12::run);
    // solutions.insert(13, &ac2025::day13::run);
    // solutions.insert(14, &ac2025::day14::run);
    // solutions.insert(15, &ac2025::day15::run);
    // solutions.insert(16, &ac2025::day16::run);
    // solutions.insert(17, &ac2025::day17::run);
    // solutions.insert(18, &ac2025::day18::run);
    // solutions.insert(19, &ac2025::day19::run);
    // solutions.insert(20, &ac2025::day20::run);
    // solutions.insert(21, &ac2025::day21::run);
    // solutions.insert(22, &ac2025::day22::run);
    // solutions.insert(23, &ac2025::day23::run);
    // solutions.insert(24, &ac2025::day24::run);
    // solutions.insert(25, &ac2025::day25::run);

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
