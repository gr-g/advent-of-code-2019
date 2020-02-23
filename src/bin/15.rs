use advent_of_code_2019::graph::{Backtracking, UnweightedGraph};
use advent_of_code_2019::grid::{Direction::{self, *}, Grid, Location};
use advent_of_code_2019::intcode::IntCode;

fn direction_code(direction: Direction) -> i64 {
    match direction {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4,
    }
}

struct ExplorerRobot {
    computer: IntCode,
    grid: Grid,
    location: Location,
}

impl ExplorerRobot {
    fn new(program: &[i64]) -> ExplorerRobot {
        let computer = IntCode::new(program);
        let mut grid = Grid::new();
        let location = Location { x: 0, y: 0 };
        grid.insert(location, '.');
        ExplorerRobot {
            computer,
            grid,
            location,
        }
    }
}

impl Backtracking for ExplorerRobot {
    type Action = Direction;

    fn list_actions(&self) -> Vec<Self::Action> {
        let mut v = Vec::new();
        for d in &[Up, Down, Left, Right] {
            if self.grid.get(&self.location.go(*d)).is_none() {
                v.push(*d);
            }
        }
        v
    }

    fn try_action(&mut self, action: &Self::Action) -> bool {
        if self.grid.get(&self.location.go(*action)).is_some() {
            // going to a cell already explored
            return false;
        }

        self.computer.input.push_back(direction_code(*action));
        self.computer.run();
        match self.computer.output.pop().unwrap() {
            0 => {
                self.grid.insert(self.location.go(*action), '#');
                false
            }
            1 => {
                self.grid.insert(self.location.go(*action), '.');
                self.location = self.location.go(*action);
                true
            }
            2 => {
                self.grid.insert(self.location.go(*action), 'O');
                self.location = self.location.go(*action);
                true
            }
            _ => panic!(),
        }
    }

    fn backtrack(&mut self, action: &Self::Action) {
        self.computer
            .input
            .push_back(direction_code(action.reverse()));
        self.computer.run();
        self.location = self.location.go(action.reverse());
    }

    fn is_solution(&self) -> bool {
        false
    }
}

// This object represents a Grid where dots are free space
// that can be walked on, forming a graph.
struct WalkableGrid(Grid);

impl UnweightedGraph<Location> for WalkableGrid {
    fn edges(&self, node: &Location) -> Vec<Location> {
        let mut v = Vec::new();

        for d in &[Up, Down, Left, Right] {
            let next_node = node.go(*d);
            if self.0.get(&next_node) == Some(&'.') {
                v.push(next_node);
            }
        }
        v
    }
}

fn solve(input: &str) -> (usize, usize) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut robot = ExplorerRobot::new(&program);

    robot.explore();
    println!("{}", robot.grid);

    let oxigen_location = *robot.grid.find('O').unwrap();
    let oxigen_distance = WalkableGrid(robot.grid).shortest_paths(oxigen_location, &[]);

    (
        oxigen_distance[&Location { x: 0, y: 0 }],
        *oxigen_distance.values().max().unwrap(),
    )
}

fn main() {
    let input = std::fs::read_to_string("input/15.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
