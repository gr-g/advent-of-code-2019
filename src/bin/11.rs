use advent_of_code_2019::intcode::IntCode;
use advent_of_code_2019::grid::{Grid, Location, Direction::*};

fn paint( program: &Vec<i64>, g: &mut Grid ) {
    let mut c = IntCode::new(program);
    let mut pos = Location{ x: 0, y: 0 };
    let mut dir = Up;
    
    while !c.is_halted() {
        match g.get(&pos) {
            Some('\u{2588}') => c.input.push_back(1),
            _ => c.input.push_back(0),
        }
        c.run();
        let turn = c.output.pop().unwrap();
        let color = c.output.pop().unwrap();
        match color {
            0 => { g.insert(pos, ' '); },
            1 => { g.insert(pos, '\u{2588}'); },
            _ => { panic!(); },
        }
        match turn {
            0 => { dir = dir.left(); pos = pos.go(dir); },
            1 => { dir = dir.right(); pos = pos.go(dir); },
            _ => panic!(),
        }
    }
}

fn solve( input: &str ) -> (usize, String) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut g = Grid::new();

    paint(&program, &mut g);
    let painted = g.symbols.len();

    let mut g = Grid::new();
    g.insert(Location{ x: 0, y: 0 }, '\u{2588}');
    paint(&program, &mut g);
    let plate = g.to_string();
    
    (painted, plate)
}

fn main() {
    let input = std::fs::read_to_string("input/11.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {}\n{}", s.0, s.1);
    println!("Time: {}ms", now.elapsed().as_millis());
}
