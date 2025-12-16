use grid::Grid;

fn parse(input: &str) -> (Vec<Grid<bool>>, Vec<((usize, usize), Vec<usize>)>) {
    let blocks_n = input.split("\n\n").count();
    let blocks = input.split("\n\n").take(blocks_n-1).enumerate().map(|(i, block)| {
        let number_line = block.lines().nth(0).unwrap();
        assert_eq!(number_line, format!("{}:", i));
        let n = block.lines().skip(1).next().unwrap().len();
        block.lines().skip(1).all(|l| l.len() == n);
        let occupancy_vector = block.lines().skip(1).map(|l| l.chars()).flatten().map(|c| {
            match c {
                '#' => true,
                '.' => false,
                c => panic!("invalid char: {}", c),
            }
        }).collect();
        Grid::from_vec(occupancy_vector, n)
    }).collect::<Vec<_>>();
    let regions = input.split("\n\n").last().unwrap().lines().map(|l| {
        let mut l_split = l.split(": ");
        let size_str = l_split.next().unwrap();
        let shapes_quantity_str = l_split.next().unwrap();
        assert!(l_split.next().is_none());
        let mut size_str_split = size_str.split("x");
        let size = (size_str_split.next().unwrap().parse::<usize>().unwrap(), size_str_split.next().unwrap().parse::<usize>().unwrap());
        assert!(l_split.next().is_none());
        let shapes_quantity = shapes_quantity_str.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (size, shapes_quantity)
    }).collect::<Vec<_>>();
    (blocks, regions)
}



pub fn solve(input: &str) -> u64 {

    let (blocks, regions) = parse(input);

    let mut min_bloc_size = vec![0; blocks.len()];
    let mut max_bloc_size = vec![0; blocks.len()];
    for (i, block) in blocks.iter().enumerate() {
        min_bloc_size[i] = block.iter().filter(|x| **x ).count();
        max_bloc_size[i] = block.rows()*block.cols();
    }


    let mut total = 0;
    for (size, quantity) in &regions {

        let mut min_size = 0;
        let mut max_size = 0;
        for (qi, q) in quantity.iter().enumerate() {
            max_size += q*max_bloc_size[qi];
            min_size += q*min_bloc_size[qi];
        }

        let area_size = size.0 * size.1;
        if area_size < min_size {
            continue;
        }
        else if area_size >= max_size {
            total += 1;
        }
        else {
            panic!("Value in between!")
        }
    }

    let ans = total;

    ans
}

pub fn run(input: &str, _test: bool) {
    let ans = solve(input);
    println!("{}", ans);
}
