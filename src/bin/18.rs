use advent_of_code_2019::graph::{Graph, UnweightedGraph};
use advent_of_code_2019::grid::{Direction::*, Grid, Location};
use std::collections::BTreeMap;
use std::collections::HashMap;

// bitmap operations to record added/missing keys
fn set_missing(keys: &[char]) -> u32 {
    let mut m = 0;
    for c in keys {
        m |= key_to_bit(*c)
    }
    m
}

fn add_key(missing_keys: u32, key: char) -> u32 {
    missing_keys & !key_to_bit(key)
}

fn has_key(missing_keys: u32, key: char) -> bool {
    missing_keys & key_to_bit(key) == 0
}

fn key_to_bit(key: char) -> u32 {
    match key {
        'a'..='z' => 1 << (key as u8 - b'a'),
        _ => panic!(),
    }
}

// Treat the map as an unweighted graph where the nodes are the
// non-wall elements.
struct WalkableGrid<'a> {
    grid: &'a Grid,
    start: &'a Location,
}

impl<'a> UnweightedGraph<Location> for WalkableGrid<'a> {
    fn edges(&self, node: &Location) -> Vec<Location> {
        let mut v = Vec::new();

        let c = self.grid.get(node).unwrap();
        if node != self.start && c != &'.' {
            // found a new item, stop here
            return v;
        }

        for d in &[Up, Down, Left, Right] {
            let next = node.go(*d);
            match self.grid.get(&next) {
                None | Some('#') => {},
                _ => { v.push(next); },
            }
        }
        v
    }
}

// Reduce the graph to the locations with keys/doors, and represent it
// as an adjacency matrix.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct AdjacencyMatrix(BTreeMap<char, BTreeMap<char, usize>>);

fn adjacency_matrix(grid: &Grid) -> AdjacencyMatrix {
    // locate the features on the map
    let item_locations: HashMap<_, _> = grid
        .symbols
        .iter()
        .filter(|&(_, c)| c != &'.' && c != &'#')
        .map(|(l, c)| (*c, *l))
        .collect();

    // Build the adjacency matrix
    let mut matrix = BTreeMap::new();
    for (i1, l1) in item_locations.iter() {
        let graph = WalkableGrid { grid, start: l1 };
        let dist = graph.shortest_paths(*l1, &[]);
        for (i2, l2) in item_locations.iter() {
            if i1 != i2 {
                if let Some(d) = dist.get(l2) {
                    matrix
                        .entry(*i1)
                        .or_insert_with(BTreeMap::new)
                        .insert(*i2, *d);
                }
            }
        }
    }
    AdjacencyMatrix(matrix)
}

// This object represents the current position
// and the keys missing (as a bitmap).
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Node {
    position: char,
    missing_keys: u32,
}

// This object represents a graph where the key/doors are nodes and
// the doors can be visited only if the corresponding key is available.
struct AdjacencyMatrixWithKeys<'a>(&'a AdjacencyMatrix, u32);

impl<'a> Graph<char> for AdjacencyMatrixWithKeys<'a> {
    fn edges(&self, node: &char) -> Vec<(char, usize)> {
        let mut v = Vec::new();

        if node.is_ascii_lowercase() && !has_key(self.1, *node) {
            // found a new item, stop here
            return v;
        }

        let reachable = (self.0).0.get(node).unwrap();

        // iterate over the keys reachable from the current position
        for (p, distance) in reachable.iter() {
            if p.is_ascii_uppercase() && !has_key(self.1, p.to_ascii_lowercase()) {
                continue;
            }
            v.push((*p, *distance));
        }
        v
    }
}

// Now we can define a graph where the nodes represent the position and
// the missing keys. Adjacent nodes are nodes where the position moves
// to a reachable key that was previously missing, and the key is recorded
// as not missing.
impl<'a> Graph<Node> for AdjacencyMatrix {
    fn edges(&self, node: &Node) -> Vec<(Node, usize)> {
        let mut v = Vec::new();

        // run a small shortest-path search to find the reachable keys
        let subgraph = AdjacencyMatrixWithKeys(&self, node.missing_keys);
        let reachable = subgraph.shortest_paths(node.position);

        // iterate over the keys reachable from the current position
        for (p, distance) in reachable.iter() {
            if p.is_ascii_lowercase() && !has_key(node.missing_keys, *p) {
                let position = *p;
                let missing_keys = add_key(node.missing_keys, *p);
                v.push((Node{ position, missing_keys }, *distance));
            }
        }
        v
    }
}

// This object represents the current position of the four robots
// and the keys missing (as a bitmap).
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Node4 {
    position: [char; 4],
    missing_keys: u32,
}

// Graph where the nodes represent the position of the four robots
// and the missing keys. Adjacent nodes are nodes where the position of
// one of the robots moves to a reachable key that was previously missing,
// and the key is recorded as not missing.
impl<'a> Graph<Node4> for AdjacencyMatrix {
    fn edges(&self, node: &Node4) -> Vec<(Node4, usize)> {
        let mut v = Vec::new();

        for i in 0..4 {
            // run a small shortest-path search to find the reachable
            // keys for robot i
            let subgraph = AdjacencyMatrixWithKeys(&self, node.missing_keys);
            let reachable = subgraph.shortest_paths(node.position[i]);

            // iterate over the keys reachable from the current position
            for (p, distance) in reachable.iter() {
                if p.is_ascii_lowercase() && !has_key(node.missing_keys, *p) {
                    let mut position = node.position;
                    position[i] = *p;
                    let missing_keys = add_key(node.missing_keys, *p);
                    v.push((Node4{ position, missing_keys }, *distance));
                }
            }
        }
        v
    }
}

fn min_distance(area: &Grid) -> usize {
    println!("{}", area);
    let matrix = adjacency_matrix(&area);
    let keys: Vec<_> = matrix.0.keys().filter(|c| c.is_ascii_lowercase()).copied().collect();
    println!("Keys to collect: {:?}", keys);

    let start = Node {
        position: '@',
        missing_keys: set_missing(&keys),
    };

    // compute the shortest paths in the graph of `Node`s
    let distances = matrix.bfs_paths(start);

    // minimum distance to a final node (a node with all keys collected)
    distances
        .iter()
        .filter(|(n, _)| n.missing_keys == 0)
        //.inspect(|x| println!("{:?}", x))
        .map(|(_, d)| *d)
        .min()
        .unwrap()
}

fn min_distance4(area: &Grid) -> usize {
    println!("{}", area);
    let matrix = adjacency_matrix(&area);
    let keys: Vec<_> = matrix.0.keys().filter(|c| c.is_ascii_lowercase()).copied().collect();
    println!("Keys to collect: {:?}", keys);

    let start = Node4 {
        position: ['1', '2', '3', '4'],
        missing_keys: set_missing(&keys),
    };

    // compute the shortest paths in the graph of `Node4`s
    let distances = matrix.bfs_paths(start);

    // minimum distance to a final node (a node with all keys collected)
    distances
        .iter()
        .filter(|(n, _)| n.missing_keys == 0)
        //.inspect(|x| println!("{:?}", x))
        .map(|(_, d)| *d)
        .min()
        .unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let mut area = Grid::create_from(input);

    let min_distance = min_distance(&area);

    // change the map, splitting it into four
    let entrance = *area.find('@').unwrap();
    area.insert(entrance, '#');
    area.insert(entrance.go(Up), '#');
    area.insert(entrance.go(Down), '#');
    area.insert(entrance.go(Left), '#');
    area.insert(entrance.go(Right), '#');
    area.insert(entrance.go(Up).go(Left), '1');
    area.insert(entrance.go(Up).go(Right), '2');
    area.insert(entrance.go(Down).go(Left), '3');
    area.insert(entrance.go(Down).go(Right), '4');

    let min_distance4 = min_distance4(&area);

    (min_distance, min_distance4)
}

fn main() {
    let input = std::fs::read_to_string("input/18.txt").unwrap();
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
        let g = Grid::create_from(
            "\
#########
#b.A.@.a#
#########",
        );
        let d = min_distance(&g);
        assert_eq!(d, 8);
    }

    #[test]
    fn example02() {
        let g = Grid::create_from(
            "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        );
        let d = min_distance(&g);
        assert_eq!(d, 86);
    }

    #[test]
    fn example03() {
        let g = Grid::create_from(
            "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
        );
        let d = min_distance(&g);
        assert_eq!(d, 132);
    }

    #[test]
    fn example04() {
        let g = Grid::create_from(
            "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
        );
        let d = min_distance(&g);
        assert_eq!(d, 136);
    }

    #[test]
    fn example05() {
        let g = Grid::create_from(
            "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
        );
        let d = min_distance(&g);
        assert_eq!(d, 81);
    }

    #[test]
    fn example06() {
        let g = Grid::create_from(
            "\
#########
#cBa@A.b#
####.#.##
####.#.##
####...##
#########",
        );
        let d = min_distance(&g);
        assert_eq!(d, 11);
    }

    #[test]
    fn example07() {
        let g = Grid::create_from(
            "\
#######
#a.#Cd#
##1#2##
#######
##3#4##
#cB#Ab#
#######",
        );
        let d = min_distance4(&g);
        assert_eq!(d, 8);
    }

    #[test]
    fn example08() {
        let g = Grid::create_from(
            "\
###############
#d.ABC.#.....a#
######1#2######
###############
######3#4######
#b.....#.....c#
###############",
        );
        let d = min_distance4(&g);
        assert_eq!(d, 24);
    }

    #[test]
    fn example09() {
        let g = Grid::create_from(
            "\
#############
#DcBa.#.GhKl#
#.###1#2#I###
#e#d#####j#k#
###C#3#4###J#
#fEbA.#.FgHi#
#############",
        );
        let d = min_distance4(&g);
        assert_eq!(d, 32);
    }

    #[test]
    fn example10() {
        let g = Grid::create_from(
            "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba1#2BcIJ#
#############
#nK.L3#4G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
        );
        let d = min_distance4(&g);
        assert_eq!(d, 72);
    }
}
