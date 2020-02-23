use advent_of_code_2019::grid::{Direction::*, Grid, Location};
use std::collections::HashMap;
use std::collections::HashSet;

// This struct stores the positions of the bugs as pairs
// (level, x-y coordinates).
struct BugsMap(HashSet<(i64, Location)>);

impl BugsMap {
    fn create_from(s: &str) -> BugsMap {
        let mut bugs = HashSet::new();
        let g = Grid::create_from(s);
        for (l, c) in g.symbols.iter() {
            if c == &'#' {
                bugs.insert((0, *l));
            }
        }
        BugsMap(bugs)
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..5 {
            for x in 0..5 {
                if self.0.contains(&(0, Location { x, y })) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }

    fn to_string_multilevel(&self) -> String {
        let mut s = String::new();
        let z_min = *self.0.iter().map(|(z, _)| z).min().unwrap_or(&0);
        let z_max = *self.0.iter().map(|(z, _)| z).max().unwrap_or(&0);
        for z in z_min..=z_max {
            s.push_str(&format!("Depth {}:\n", z));
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        s.push('?');
                    } else if self.0.contains(&(z, Location { x, y })) {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
                s.push('\n');
            }
        }
        s
    }

    fn biodiversity(&self) -> usize {
        let mut rating = 0;
        let mut points = 1;
        for y in 0..5 {
            for x in 0..5 {
                if self.0.contains(&(0, Location { x, y })) {
                    rating += points;
                }
                points *= 2;
            }
        }
        rating
    }

    fn advance(&mut self) {
        // count neighbouring bugs
        let mut count = HashMap::new();
        for (level, location) in self.0.iter() {
            count.entry((*level, *location)).or_insert(0);
            match location.x {
                0 => { *count.entry((*level, location.go(Right))).or_insert(0) += 1; },
                4 => { *count.entry((*level, location.go(Left))).or_insert(0) += 1; },
                1|2|3 => {
                    *count.entry((*level, location.go(Left))).or_insert(0) += 1;
                    *count.entry((*level, location.go(Right))).or_insert(0) += 1;
                },
                _ => { panic!(); },
            }
            match location.y {
                0 => { *count.entry((*level, location.go(Down))).or_insert(0) += 1; },
                4 => { *count.entry((*level, location.go(Up))).or_insert(0) += 1; },
                1|2|3 => {
                    *count.entry((*level, location.go(Up))).or_insert(0) += 1;
                    *count.entry((*level, location.go(Down))).or_insert(0) += 1;
                },
                _ => { panic!(); },
            }
        }

        // update bug locations
        for (bug, n) in count.into_iter() {
            if self.0.contains(&bug) {
                match n {
                    1 => {},
                    _ => { self.0.remove(&bug); },
                }
            } else {
                match n {
                    1|2 => { self.0.insert(bug); },
                    _   => {},
                }
            }
        }
    }
    
    fn advance_multilevel( &mut self ) {
        // count neighbouring bugs
        let mut count = HashMap::new();
        for (level, location) in self.0.iter() {
            count.entry((*level, *location)).or_insert(0);
            match (location.x, location.y) {
                (0,_) => {
                    *count.entry((*level - 1, Location{ x: 1, y: 2 })).or_insert(0) += 1;
                    *count.entry((*level, location.go(Right))).or_insert(0) += 1;
                },
                (4,_) => {
                    *count.entry((*level, location.go(Left))).or_insert(0) += 1;
                    *count.entry((*level - 1, Location{ x: 3, y: 2 })).or_insert(0) += 1;
                },
                (_,0)|(_,1)|(_,3)|(_,4) => {
                    *count.entry((*level, location.go(Left))).or_insert(0) += 1;
                    *count.entry((*level, location.go(Right))).or_insert(0) += 1;
                },
                (1,2) => {
                    *count.entry((*level, location.go(Left))).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 1 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 2 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 3 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 4 })).or_insert(0) += 1;
                },
                (3,2) => {
                    *count.entry((*level + 1, Location{ x: 4, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 1 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 2 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 3 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level, location.go(Right))).or_insert(0) += 1;
                },
                _ => { panic!(); },
            }
            match (location.x, location.y) {
                (_,0) => {
                    *count.entry((*level - 1, Location{ x: 2, y: 1 })).or_insert(0) += 1;
                    *count.entry((*level, location.go(Down))).or_insert(0) += 1;
                },
                (_,4) => {
                    *count.entry((*level, location.go(Up))).or_insert(0) += 1;
                    *count.entry((*level - 1, Location{ x: 2, y: 3 })).or_insert(0) += 1;
                },
                (0,_)|(1,_)|(3,_)|(4,_) => {
                    *count.entry((*level, location.go(Up))).or_insert(0) += 1;
                    *count.entry((*level, location.go(Down))).or_insert(0) += 1;
                },
                (2,1) => {
                    *count.entry((*level, location.go(Up))).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 0, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 1, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 2, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 3, y: 0 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 0 })).or_insert(0) += 1;
                },
                (2,3) => {
                    *count.entry((*level + 1, Location{ x: 0, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 1, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 2, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 3, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level + 1, Location{ x: 4, y: 4 })).or_insert(0) += 1;
                    *count.entry((*level, location.go(Down))).or_insert(0) += 1;
                },
                _ => { panic!(); },
            }
        }

        // update bug locations
        for (bug, n) in count.into_iter() {
            if self.0.contains(&bug) {
                match n {
                    1 => {}
                    _ => {
                        self.0.remove(&bug);
                    }
                }
            } else {
                match n {
                    1 | 2 => {
                        self.0.insert(bug);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn solve(input: &str, n: usize) -> (usize, usize) {
    let mut bugs = BugsMap::create_from(input);
    println!("{}", bugs.to_string());

    let mut seen_b = HashSet::new();
    let mut current_b = bugs.biodiversity();
    while !seen_b.contains(&current_b) {
        seen_b.insert(current_b);
        bugs.advance();
        current_b = bugs.biodiversity();
    }
    //println!("{}", bugs.to_string());

    let mut bugs = BugsMap::create_from(input);
    println!("{}", bugs.to_string_multilevel());
    for _ in 0..n {
        bugs.advance_multilevel();
    }
    //println!("{}", bugs.to_string_multilevel());

    (current_b, bugs.0.len())
}

fn main() {
    let input = std::fs::read_to_string("input/24.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 200);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let mut bugs = BugsMap::create_from(
            "\
....#
#..#.
#..##
..#..
#....",
        );
        bugs.advance();
        assert_eq!(
            bugs.to_string(),
            "\
#..#.
####.
###.#
##.##
.##..
"
        );
        bugs.advance();
        assert_eq!(
            bugs.to_string(),
            "\
#####
....#
....#
...#.
#.###
"
        );
        bugs.advance();
        assert_eq!(
            bugs.to_string(),
            "\
#....
####.
...##
#.##.
.##.#
"
        );
        bugs.advance();
        assert_eq!(
            bugs.to_string(),
            "\
####.
....#
##..#
.....
##...
"
        );
        let mut seen_b = HashSet::new();
        let mut current_b = bugs.biodiversity();
        while !seen_b.contains(&current_b) {
            seen_b.insert(current_b);
            bugs.advance();
            current_b = bugs.biodiversity();
        }
        assert_eq!(
            bugs.to_string(),
            "\
.....
.....
.....
#....
.#...
"
        );
        assert_eq!(current_b, 2129920);
    }

    #[test]
    fn example02() {
        let mut bugs = BugsMap::create_from(
            "\
....#
#..#.
#.?##
..#..
#....",
        );
        for _ in 0..10 {
            bugs.advance_multilevel();
        }
        assert_eq!(
            bugs.to_string_multilevel(),
            "\
Depth -5:
..#..
.#.#.
..?.#
.#.#.
..#..
Depth -4:
...#.
...##
..?..
...##
...#.
Depth -3:
#.#..
.#...
..?..
.#...
#.#..
Depth -2:
.#.##
....#
..?.#
...##
.###.
Depth -1:
#..##
...##
..?..
...#.
.####
Depth 0:
.#...
.#.##
.#?..
.....
.....
Depth 1:
.##..
#..##
..?.#
##.##
#####
Depth 2:
###..
##.#.
#.?..
.#.##
#.#..
Depth 3:
..###
.....
#.?..
#....
#...#
Depth 4:
.###.
#..#.
#.?..
##.#.
.....
Depth 5:
####.
#..#.
#.?#.
####.
.....
"
        );
    }
}
