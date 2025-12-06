use std::fs;

enum Oper {
    Sum,
    Mult,
}

fn parse_operations(line: &str) -> Vec<Oper> {
    line.split_whitespace().map(|s| {
        match s {
            "*" => Oper::Mult,
            "+" => Oper::Sum,
            c => panic!("invalid operation symbol {}", c),
        }
    }).collect::<Vec<_>>()
}

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Oper>) {
    let n_numbers = input.lines().count() - 1;
    let number_lines = input.lines().take(n_numbers).map(|line| {
        line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let operations = parse_operations(input.lines().nth(n_numbers).unwrap());
    assert!(number_lines.iter().all(|n| n.len() == operations.len()));
    (number_lines, operations)
}

fn parse2(input: &str) -> (Vec<Vec<u32>>, Vec<Oper>) {
    let n_numbers = input.lines().count() - 1;
    let number_lines = input.lines().take(n_numbers).map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let operations = parse_operations(input.lines().nth(n_numbers).unwrap());
    let max_number_of_symbols = number_lines.iter().map(|l| l.len()).max().unwrap() + 1;
    let mut number_sets = vec![];
    let mut number_set = vec![];
    for i in 0..max_number_of_symbols {
        let digits = number_lines.iter().map(|l| l.get(i).unwrap_or(&' ')).collect::<Vec<_>>();
        if digits.iter().all(|&d| d == &' ' ) {
            if !number_set.is_empty() {
                number_sets.push(std::mem::take(&mut number_set));
            }
            continue
        }
        let mut number = 0;
        for digit in digits {
            if digit == &' ' {
                continue;
            }
            number *= 10;
            number += digit.to_digit(10).unwrap();
        }
        number_set.push(number);
    }
    assert_eq!(number_sets.len(), operations.len());
    (number_sets, operations)
}

pub fn solve(input: &str) -> (u64, u64) {

    let (number_lines, operations) = parse(input);
    let ans1 = (0..operations.len()).map(|i| {
        match operations[i] {
            Oper::Sum => (0..number_lines.len()).map(|j| number_lines[j][i] as u64).sum::<u64>(),
            Oper::Mult => (0..number_lines.len()).map(|j| number_lines[j][i] as u64).product(),
        }
    }).sum();

    let (number_lines, operations) = parse2(input);
    let ans2 = number_lines.iter().zip(operations).map(|(numbers, operation)| {
        match operation {
            Oper::Sum => numbers.iter().map(|&x| x as u64).sum::<u64>(),
            Oper::Mult => numbers.iter().map(|&x| x as u64).product(),
        }
    }).sum();

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day06_0.txt")
                    .unwrap()
                    .as_str()
            ), (4277556, 3263827));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
