use std::cmp::Ordering;
use advent_of_code_2019::intcode::IntCode;
use advent_of_code_2019::grid::{Grid, Location};

struct Game {
    computer: IntCode,
    screen: Grid,
    paddle: (i64, i64),
    ball: (i64, i64),
    score: i64,
}

impl Game {
    fn new( program: &[i64] ) -> Game {
        Game{
            computer: IntCode::new(program),
            screen: Grid::new(),
            paddle: (0, 0),
            ball: (0, 0),
            score: 0,
        }
    }

    fn set_move( &mut self, m: i64 ) {
        self.computer.input.push_back(m);
    }

    fn run( &mut self ) {
        self.computer.run();
        for cell in self.computer.output.chunks_exact(3) {
            match cell {
                &[-1, 0, v] => { self.score = v; },
                &[x, y, 0] => { self.screen.remove(&Location{ x, y }); },
                &[x, y, 1] => { self.screen.insert(Location{ x, y }, '#'); },
                &[x, y, 2] => { self.screen.insert(Location{ x, y }, '\u{2588}'); },
                &[x, y, 3] => {
                    self.screen.insert(Location{ x, y }, '_');
                    self.paddle = (x, y);
                },
                &[x, y, 4] => {
                    self.screen.insert(Location{ x, y }, 'o');
                    self.ball = (x, y);
                },
                _ => panic!()
            }
        }
        self.computer.output.clear();
        // uncomment to show the game
        //println!("{}", self.screen);
        //std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn solve( input: &str ) -> (usize, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut g = Game::new(&program);
    g.run();

    let blocks = g.screen.symbols.values().filter(|&x| x == &'\u{2588}').count();
    
    let mut g = Game::new(&program);
    g.computer.memory[0] = 2;
    while !g.computer.is_halted() {
        g.run();
        match i64::cmp(&g.paddle.0, &g.ball.0) {
            Ordering::Less => g.set_move(1),
            Ordering::Equal => g.set_move(0),
            Ordering::Greater => g.set_move(-1),
        }
    }
    
    (blocks, g.score)
}

fn main() {
    let input = std::fs::read_to_string("input/13.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
