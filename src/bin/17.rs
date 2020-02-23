use advent_of_code_2019::graph::Backtracking;
use advent_of_code_2019::grid::{Direction::*, Grid};
use advent_of_code_2019::intcode::IntCode;

fn alignment_parameters(area: &Grid) -> i64 {
    area.symbols
        .iter()
        .filter(|(location, value)| {
            **value == '#'
                && area.get(&location.go(Up)) == Some(&'#')
                && area.get(&location.go(Down)) == Some(&'#')
                && area.get(&location.go(Left)) == Some(&'#')
                && area.get(&location.go(Right)) == Some(&'#')
        })
        .map(|(location, _)| location.x * location.y)
        .sum()
}

// This struct represents the (attempted) compression of a sequence of
// elements by defining three sub-sequences (A/B/C) and using them to
// represent the full sequence. A solution is found by exploring possible
// "actions", with backtracking: an "action" consists in compressing the
// next n elements using one of the matching sub-sequences, continuing
// until the full sequence is consumed.
struct Compression<'a, T> {
    source: &'a [T],           // the sequence to compress
    max_len: usize,            // maximum length of the sub-sequences
    index: usize,              // string compressed up to this point
    subs: [(usize, usize); 3], // the start and length of the 3 chosen sub-sequences
                               // zero length means that the sub-sequence is still undefined
    compressed: Vec<String>,   // the sub-sequences used until now, represented as "A"/"B"/"C"
}

impl<'a, T> Compression<'a, T> {
    fn new(source: &'a [T], max_len: usize) -> Compression<'a, T> {
        Compression {
            source,
            max_len,
            index: 0,
            subs: [(0, 0); 3],
            compressed: Vec::new(),
        }
    }

    fn get_sub(&self, sub: usize) -> &'a [T] {
        let (pos, len) = self.subs[sub];
        &self.source[pos..pos + len]
    }
}

struct CompressionStep {
    sub: usize, // the sub-sequence to use
    len: usize, // the number of elements to take
}

impl<'a, T: Eq> Backtracking for Compression<'a, T> {
    type Action = CompressionStep;

    fn list_actions(&self) -> Vec<CompressionStep> {
        let mut v = Vec::new();
        for sub in 0..self.subs.len() {
            let (pos, len) = self.subs[sub];
            if len == 0 {
                // sub-sequence not defined yet, try various lengths
                for len in 1..=self.max_len {
                    v.push(CompressionStep { sub, len });
                }
                return v;
            } else {
                // check if this sub-sequence matches the next len elements
                if self.index + len <= self.source.len()
                    && self.source[self.index..self.index + len] == self.source[pos..pos + len]
                {
                    v.push(CompressionStep { sub, len });
                }
            }
        }
        v
    }

    fn try_action(&mut self, action: &CompressionStep) -> bool {
        let CompressionStep { sub, len } = *action;
        if self.subs[sub].1 == 0 {
            // sub-sequence was not defined, define it now
            self.subs[sub] = (self.index, len);
        }
        self.index += len;
        self.compressed
            .push(((b'A' + sub as u8) as char).to_string());
        true
    }

    fn backtrack(&mut self, action: &CompressionStep) {
        let CompressionStep { sub, len } = *action;
        self.index -= len;
        self.compressed.pop();
        if self.index == self.subs[sub].0 {
            // un-define the sub-sequence
            self.subs[sub] = (0, 0);
        }
    }

    fn is_solution(&self) -> bool {
        self.index == self.source.len()
    }
}

fn plan_path(area: &Grid) -> Vec<String> {
    // find the robot (position and direction) in the map
    let (mut pos, mut dir) = area
        .symbols
        .iter()
        .find_map(|(location, value)| match value {
            '^' => Some((*location, Up)),
            'v' => Some((*location, Down)),
            '<' => Some((*location, Left)),
            '>' => Some((*location, Right)),
            _ => None,
        })
        .unwrap();

    let mut path = Vec::new();
    loop {
        // find new direction
        let next_turn;
        if area.get(&pos.go(dir.right())) == Some(&'#') {
            dir = dir.right();
            next_turn = 'R';
        } else if area.get(&pos.go(dir.left())) == Some(&'#') {
            dir = dir.left();
            next_turn = 'L';
        } else {
            break;
        }

        // advance as much as possible
        let mut next_steps = 0;
        while area.get(&pos.go(dir)) == Some(&'#') {
            pos = pos.go(dir);
            next_steps += 1;
        }

        path.push(format!("{},{}", next_turn, next_steps));
    }
    path
}

fn solve(input: &str) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    // map the surroundings
    let mut computer = IntCode::new(&program);
    let area = Grid::create_from(&computer.run_ascii_command("").0);

    // compute the sum of the alignment parameters
    let alignment = alignment_parameters(&area);

    // plan the path of the robot
    let path = plan_path(&area);
    println!("Planned path for the robot: {:?}", path);
    println!();

    // compress the path
    let mut p = Compression::new(&path, 5);
    p.explore().expect("no suitable compression found");

    // send the robot on its way
    computer.reset(&program);
    computer.memory[0] = 2;
    computer.run_ascii_command("");
    computer.run_ascii_command(&p.compressed.join(","));
    computer.run_ascii_command(&p.get_sub(0).join(","));
    computer.run_ascii_command(&p.get_sub(1).join(","));
    computer.run_ascii_command(&p.get_sub(2).join(","));
    let (_, specks) = computer.run_ascii_command("n");

    (alignment, specks[0])
}

fn main() {
    let input = std::fs::read_to_string("input/17.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let area = Grid::create_from(
            "\
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..",
        );
        assert_eq!(alignment_parameters(&area), 76);
    }

    #[test]
    fn example02() {
        let area = Grid::create_from(
            "\
#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......",
        );
        let path = plan_path(&area);
        assert_eq!(
            path.join(","),
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2".to_string()
        );
        let mut p = Compression::new(&path, 3);
        p.explore().unwrap();
        println!("Path compressed as: {:?}", p.compressed);
        println!("A: {:?}", p.get_sub(0));
        println!("B: {:?}", p.get_sub(1));
        println!("C: {:?}", p.get_sub(2));
        assert_eq!(p.compressed.join(","), "A,B,C,B,A,C".to_string());
        assert_eq!(p.get_sub(0).join(","), "R,8,R,8".to_string());
        assert_eq!(p.get_sub(1).join(","), "R,4,R,4,R,8".to_string());
        assert_eq!(p.get_sub(2).join(","), "L,6,L,2".to_string());
    }
}
