use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Rectangle {
    area: u64,
    _p1: (u64, u64),
    _p2: (u64, u64),
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
}

impl Rectangle {
    fn new(p1: (u64, u64), p2: (u64, u64)) -> Self {
        let area = (p1.0.abs_diff(p2.0)+1)*(p1.1.abs_diff(p2.1)+1);
        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);
        Self{area, _p1: p1, _p2: p2, min_x,max_x,min_y,max_y}
    }
}

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Line {
    same: u64,
    other_min: u64,
    other_max: u64,
    direction: Direction,
}

impl Line {
    fn new(p1: (u64, u64), p2: (u64, u64)) -> Self {
        if p1.0 == p2.0 {
            Self{same: p1.0, other_min: p1.1.min(p2.1), other_max: p1.1.max(p2.1), direction: Direction::Vertical}
        }
        else if p1.1 == p2.1 {
            Self{same: p1.1, other_min: p1.0.min(p2.0), other_max: p1.0.max(p2.0), direction: Direction::Horizontal}
        }
        else {
            panic!("The line between {:?} and {:?} is neither horizontal nor vertical", p1, p2);
        }
    }
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input.lines().map(|line| {
        let mut split = line.split(",");
        let r = (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap());
        assert!(split.next().is_none());
        r
    }).collect()
}

fn find_biggest_inner_rectangle<'a>(rectangles: &'a Vec<Rectangle>, vert_lines: &Vec<Line>, hor_lines: &Vec<Line>) -> Option<&'a Rectangle> {

    'rectangle: for rectangle in rectangles {

        //check if any vertical lines crosses the rectangle
        let x_ind_start = vert_lines.binary_search_by(|p| (p.same as f64).total_cmp(&(rectangle.min_x as f64 + 0.1))).err().unwrap();
        let x_ind_end = vert_lines.binary_search_by(|p| (p.same as f64).total_cmp(&(rectangle.max_x as f64 - 0.1))).err().unwrap();
        for xi in x_ind_start..x_ind_end {
            if !(vert_lines[xi].other_max <= rectangle.min_y || vert_lines[xi].other_min >= rectangle.max_y) {
                continue 'rectangle;
            }
        }

        //check if any horizontal lines crosses the rectangle
        let y_ind_start = hor_lines.binary_search_by(|p| (p.same as f64).total_cmp(&(rectangle.min_y as f64 + 0.1))).err().unwrap();
        let y_ind_end = hor_lines.binary_search_by(|p| (p.same as f64).total_cmp(&(rectangle.max_y as f64 - 0.1))).err().unwrap();
        for yi in y_ind_start..y_ind_end {
            if !(hor_lines[yi].other_max <= rectangle.min_x || hor_lines[yi].other_min >= rectangle.max_x) {
                continue 'rectangle;
            }
        }

        //make probe if it is inner or outer rectangle
        let x_probe = rectangle.min_x as f64 + 0.1;
        let y_probe = rectangle.min_y as f64 + 0.1;
        let n_hor_lines_on_top = hor_lines.iter().filter(|l| (l.same as f64) < y_probe).filter(|l| (l.other_min as f64) < x_probe && (l.other_max as f64) > x_probe).count();
        if n_hor_lines_on_top % 2 == 1 {
            return Some(rectangle);
        }
    }

    None
}

pub fn solve(input: &str) -> (u64, u64) {

    let coords = parse(input);

    let mut rectangles = coords.iter().combinations(2).map(|pairs| {
        Rectangle::new(*pairs[0], *pairs[1])
    }).collect::<Vec<_>>();

    // dbg!(&areas);
    rectangles.sort_by_key(|r| r.area);
    rectangles.reverse();

    let ans1 = rectangles[0].area;

    let mut vert_lines = vec![];
    let mut hor_lines = vec![];
    for i in 0..coords.len() {
        let p1 = coords[i];
        let p2 = coords[(i+1) % coords.len()];

        let line = Line::new(p1, p2);
        match line.direction {
            Direction::Horizontal => hor_lines.push(line),
            Direction::Vertical => vert_lines.push(line),
        }
    }
    vert_lines.sort_by_key(|l| l.same);
    hor_lines.sort_by_key(|l| l.same);


    let ans2 = find_biggest_inner_rectangle(&rectangles, &vert_lines, &hor_lines).unwrap().area;

    (ans1,ans2)
}

pub fn run(input: &str, test: bool) {
    if test {
        assert_eq!(
            solve(
                fs::read_to_string("test_data/day09_0.txt")
                    .unwrap()
                    .as_str()
            ), (50, 24));
    }
    let ans = solve(input);
    println!("{}, {}", ans.0, ans.1);
}
