use advent_of_code_2019::graph::UnweightedGraph;
use advent_of_code_2019::grid::{Direction::*, Grid, Location};
use std::collections::HashMap;

struct GridWithPortals {
    grid: Grid,
    inner_tag: HashMap<Location, String>,
    inner_portal: HashMap<String, Location>,
    outer_tag: HashMap<Location, String>,
    outer_portal: HashMap<String, Location>,
}

impl GridWithPortals {
    fn create_from(s: &str) -> GridWithPortals {
        let grid = Grid::create_from(s);
        let mut inner_tag = HashMap::new();
        let mut inner_portal = HashMap::new();
        let mut outer_tag = HashMap::new();
        let mut outer_portal = HashMap::new();
        for (l, c) in grid.symbols.iter() {
            if c == &'.' {
                for d in &[Up, Down, Left, Right] {
                    if let Some(c1) = grid.get(&l.go(*d)) {
                        if c1.is_ascii_uppercase() {
                            let c2 = grid.get(&l.go(*d).go(*d)).unwrap();
                            let s = {
                                if *d == Right || *d == Down {
                                    let mut s = c1.to_string();
                                    s.push(*c2);
                                    s
                                } else {
                                    let mut s = c2.to_string();
                                    s.push(*c1);
                                    s
                                }
                            };
                            if l.x < grid.x_min() + 3
                                || l.x > grid.x_max() - 3
                                || l.y < grid.y_min() + 3
                                || l.y > grid.y_max() - 3
                            {
                                outer_tag.insert(*l, s.clone());
                                outer_portal.insert(s, *l);
                            } else {
                                inner_tag.insert(*l, s.clone());
                                inner_portal.insert(s, *l);
                            }
                        }
                    }
                }
            }
        }
        GridWithPortals {
            grid,
            inner_tag,
            inner_portal,
            outer_tag,
            outer_portal,
        }
    }
}

impl UnweightedGraph<Location> for GridWithPortals {
    fn edges(&self, node: &Location) -> Vec<Location> {
        let mut v = Vec::new();

        // regular move
        for d in &[Up, Down, Left, Right] {
            let next_node = node.go(*d);
            if self.grid.get(&next_node) == Some(&'.') {
                v.push(next_node);
            }
        }
        // move from inner portal to outer portal
        if let Some(s) = self.inner_tag.get(node) {
            if let Some(next_node) = self.outer_portal.get(s) {
                v.push(*next_node);
            }
        }
        // move from outer portal to inner portal
        if let Some(s) = self.outer_tag.get(node) {
            if let Some(next_node) = self.inner_portal.get(s) {
                v.push(*next_node);
            }
        }
        v
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct LayeredLocation {
    location: Location,
    layer: usize,
}

impl UnweightedGraph<LayeredLocation> for GridWithPortals {
    fn edges(&self, node: &LayeredLocation) -> Vec<LayeredLocation> {
        let mut v = Vec::new();

        // regular move
        for d in &[Up, Down, Left, Right] {
            let next_node = LayeredLocation {
                location: node.location.go(*d),
                layer: node.layer,
            };
            if self.grid.get(&next_node.location) == Some(&'.') {
                v.push(next_node);
            }
        }
        // move from inner portal to outer portal
        if let Some(s) = self.inner_tag.get(&node.location) {
            if let Some(l) = self.outer_portal.get(s) {
                let next_node = LayeredLocation {
                    location: *l,
                    layer: node.layer + 1,
                };
                v.push(next_node);
            }
        }
        // move from outer portal to inner portal
        if node.layer > 0 {
            if let Some(s) = self.outer_tag.get(&node.location) {
                if let Some(l) = self.inner_portal.get(s) {
                    let next_node = LayeredLocation {
                        location: *l,
                        layer: node.layer - 1,
                    };
                    v.push(next_node);
                }
            }
        }
        v
    }
}

fn solve(input: &str) -> (usize, usize) {
    let g = GridWithPortals::create_from(input);
    println!("{}", g.grid);

    let start = g.outer_portal["AA"];
    let target = g.outer_portal["ZZ"];
    let distances = g.shortest_paths(start, &[target]);

    let layered_start = LayeredLocation {
        location: g.outer_portal["AA"],
        layer: 0,
    };
    let layered_target = LayeredLocation {
        location: g.outer_portal["ZZ"],
        layer: 0,
    };
    let layered_distances = g.shortest_paths(layered_start, &[layered_target]);

    (distances[&target], layered_distances[&layered_target])
}

fn main() {
    let input = std::fs::read_to_string("input/20.txt").unwrap();
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
        let g = GridWithPortals::create_from(
            "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
        );
        let start = g.outer_portal["AA"];
        let target = g.outer_portal["ZZ"];
        let distances = g.shortest_paths(start, &[target]);
        assert_eq!(distances[&target], 23);
        let layered_start = LayeredLocation {
            location: g.outer_portal["AA"],
            layer: 0,
        };
        let layered_target = LayeredLocation {
            location: g.outer_portal["ZZ"],
            layer: 0,
        };
        let layered_distances = g.shortest_paths(layered_start, &[layered_target]);
        assert_eq!(layered_distances[&layered_target], 26);
    }

    #[test]
    fn example02() {
        let g = GridWithPortals::create_from(
            "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",
        );
        let start = g.outer_portal["AA"];
        let target = g.outer_portal["ZZ"];
        let distances = g.shortest_paths(start, &[target]);
        assert_eq!(distances[&target], 58);
    }

    #[test]
    fn example03() {
        let g = GridWithPortals::create_from(
            "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ",
        );
        let layered_start = LayeredLocation {
            location: g.outer_portal["AA"],
            layer: 0,
        };
        let layered_target = LayeredLocation {
            location: g.outer_portal["ZZ"],
            layer: 0,
        };
        let layered_distances = g.shortest_paths(layered_start, &[layered_target]);
        assert_eq!(layered_distances[&layered_target], 396);
    }
}
