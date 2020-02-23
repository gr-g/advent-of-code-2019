use advent_of_code_2019::grid::{Grid, Location};
use advent_of_code_2019::intcode::IntCode;

fn check_location(program: &[i64], x: i64, y: i64) -> bool {
    let mut ic = IntCode::new(program);
    ic.input.push_back(x);
    ic.input.push_back(y);
    ic.run();
    ic.output.pop().unwrap() != 0
}

fn solve(input: &str) -> (usize, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut g = Grid::new();
    let mut affected = 0;
    for x in 0..50 {
        for y in 0..50 {
            if check_location(&program, x, y) {
                affected += 1;
                g.insert(Location { x, y }, '#');
            }
        }
    }
    println!("{}", g);

    let mut x = 0;
    let mut y = 0;

    while !check_location(&program, x, y + 99) {
        x += 1;
        while !check_location(&program, x + 99, y) {
            y += 1;
        }
    }

    (affected, x * 10000 + y)
}

fn main() {
    let input = std::fs::read_to_string("input/19.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
