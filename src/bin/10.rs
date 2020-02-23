use advent_of_code_2019::grid::{Grid, Location};
use std::cmp::Ordering;
use std::collections::HashSet;

fn gcd(mut m: i64, mut n: i64) -> i64 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

// This structure encodes an "amount of rotation" as three integers
// (n, a, b), where n is a number of half rotations, and a, b with
// a >= 0 and a²+b² > 0 encode an additional rotation by alpha:
//   (a, b) / (a²+b²) = (sin(alpha), cos(alpha))
#[derive(Debug, PartialEq, Eq)]
struct IntegerRotation {
    n: u64,
    a: i64,
    b: i64,
}

impl PartialOrd for IntegerRotation {
    fn partial_cmp(&self, other: &IntegerRotation) -> Option<Ordering> {
        Some(u64::cmp(&self.n, &other.n).then(i64::cmp(&(other.b * self.a), &(self.b * other.a))))
    }
}

struct AsteroidMap(HashSet<Location>);

impl AsteroidMap {
    fn create_from(s: &str) -> AsteroidMap {
        let mut asteroids = HashSet::new();
        let g = Grid::create_from(s);
        for (l, c) in g.symbols.iter() {
            if c == &'#' {
                asteroids.insert(*l);
            }
        }
        AsteroidMap(asteroids)
    }

    fn asteroids_on_path(&self, from: &Location, to: &Location) -> u64 {
        let mut res = 0;
        let x_diff = to.x - from.x;
        let y_diff = to.y - from.y;
        let gcd = gcd(x_diff, y_diff);

        for i in 1..gcd {
            let inside_location = Location {
                x: from.x + x_diff / gcd * i,
                y: from.y + y_diff / gcd * i,
            };
            if self.0.contains(&inside_location) {
                res += 1;
            }
        }
        res
    }

    fn direct_line(&self, from: &Location, to: &Location) -> bool {
        self.asteroids_on_path(from, to) == 0
    }

    fn angle_from_vertical(&self, from: &Location, to: &Location) -> IntegerRotation {
        let x_diff = to.x - from.x;
        let y_diff = -(to.y - from.y);

        if x_diff > 0 || (x_diff == 0 && y_diff > 0) {
            IntegerRotation {
                n: self.asteroids_on_path(from, to) * 2,
                a: x_diff,
                b: y_diff,
            }
        } else {
            IntegerRotation {
                n: self.asteroids_on_path(from, to) * 2 + 1,
                a: -x_diff,
                b: -y_diff,
            }
        }
    }
}

fn max_visibility(m: &AsteroidMap) -> (&Location, usize) {
    let mut best_loc = (&Location { x: 0, y: 0 }, 0);
    for a in m.0.iter() {
        let visible_from_a =
            m.0.iter()
                .filter(|b| (*b != a) && m.direct_line(a, b))
                .count();

        if visible_from_a > best_loc.1 {
            best_loc = (a, visible_from_a);
        }
    }
    best_loc
}

fn nth_target<'a>(m: &'a AsteroidMap, station: &Location, n: usize) -> &'a Location {
    let mut angles: Vec<_> =
        m.0.iter()
            .filter(|b| (b != &station))
            .map(|b| (b, m.angle_from_vertical(station, b)))
            .collect();
    angles.sort_by(|(_, ang1), (_, ang2)| ang1.partial_cmp(ang2).unwrap());
    angles.get(n - 1).unwrap().0
}

fn solve(input: &str, n: usize) -> (usize, i64) {
    let m = AsteroidMap::create_from(input);

    let (best_loc, visible_asteroids) = max_visibility(&m);
    let target_asteroid = nth_target(&m, best_loc, n);
    let nth_target = target_asteroid.x * 100 + target_asteroid.y;

    (visible_asteroids, nth_target)
}

fn main() {
    let input = std::fs::read_to_string("input/10.txt").unwrap();
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
        let m = AsteroidMap::create_from(
            "\
.#..#
.....
#####
....#
...##",
        );
        assert_eq!(max_visibility(&m), (&Location { x: 3, y: 4 }, 8));
    }

    #[test]
    fn example02() {
        let m = AsteroidMap::create_from(
            "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        );
        assert_eq!(max_visibility(&m), (&Location { x: 5, y: 8 }, 33));
    }

    #[test]
    fn example03() {
        let m = AsteroidMap::create_from(
            "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
        );
        assert_eq!(max_visibility(&m), (&Location { x: 1, y: 2 }, 35));
    }

    #[test]
    fn example04() {
        let m = AsteroidMap::create_from(
            "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
        );
        assert_eq!(max_visibility(&m), (&Location { x: 6, y: 3 }, 41));
    }

    #[test]
    fn example05() {
        let m = AsteroidMap::create_from(
            "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        assert_eq!(max_visibility(&m), (&Location { x: 11, y: 13 }, 210));

        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 1),
            &Location { x: 11, y: 12 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 2),
            &Location { x: 12, y: 1 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 3),
            &Location { x: 12, y: 2 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 10),
            &Location { x: 12, y: 8 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 20),
            &Location { x: 16, y: 0 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 50),
            &Location { x: 16, y: 9 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 100),
            &Location { x: 10, y: 16 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 199),
            &Location { x: 9, y: 6 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 200),
            &Location { x: 8, y: 2 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 201),
            &Location { x: 10, y: 9 }
        );
        assert_eq!(
            nth_target(&m, max_visibility(&m).0, 299),
            &Location { x: 11, y: 1 }
        );
    }
}
