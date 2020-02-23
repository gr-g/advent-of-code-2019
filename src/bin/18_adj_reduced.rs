use std::collections::HashMap;
use std::collections::BTreeMap;
use advent_of_code_2019::grid::{Grid, Location, Direction::*};
use advent_of_code_2019::graph::{UnweightedGraph, Graph};

// This object represents a graph where the nodes are the non-wall elements
// of the grid, and paths can pass through doors only if the associated
// key is available.
//struct WalkableGrid<'a> {
//    grid: &'a Grid,
//    key_locations: &'a HashMap<char, GridLocation>,
//    missing_keys: MissingKeys,
//}

// Treat the map as a graph where the nodes are the non-wall elements
struct WalkableGrid<'a>{
    grid: &'a Grid,
    start: &'a Location,
}

impl<'a> UnweightedGraph<Location> for WalkableGrid<'a> {
    fn edges( &self, node: &Location ) -> Vec<Location> {
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

fn adjacency_matrix( grid: &Grid ) -> AdjacencyMatrix {
    // locate the features on the map
    let item_locations: HashMap<_, _> = grid.symbols
        .iter()
        .filter(|&(_, c)| c != &'.' && c != &'#')
        .map(|(l, c)| (*c, *l))
        .collect();

    // Build the adjacency matrix
    let mut matrix = BTreeMap::new();
    for (i1, l1) in item_locations.iter() {
        let graph = WalkableGrid{ grid, start: l1 };
        let dist = graph.shortest_paths(*l1, &[]);
        for (i2, l2) in item_locations.iter() {
            if i1 != i2 {
                if let Some(d) = dist.get(l2) {
                    matrix.entry(*i1).or_insert(BTreeMap::new()).insert(*i2, *d);
                }
            }
        }
    }
    AdjacencyMatrix(matrix)
}

// Reduce the adjacency matrix by moving from the current position to item k
// and recomputing the distances as:
//   dist(x,y) = min( dist(x,y), dist(x,k) + dist(k,y) )
fn reduce( adj: &mut AdjacencyMatrix, item: &char ) {
    let dist_k = adj.0.get(item);
    if dist_k.is_none() {
        return;
    }
    let dist_k = dist_k.unwrap().clone();
    let mut to_remove = Vec::new();
    
    for (x, dist_x) in adj.0.iter_mut() {
        if let Some(dist_xk) = dist_x.remove(item) {
            for (y, dist_ky) in dist_k.iter() {
                if x != y {
                    match dist_x.get_mut(y) {
                        Some(dist_xy) => { *dist_xy = std::cmp::min(*dist_xy, dist_xk + *dist_ky); },
                        None => { dist_x.insert(*y, dist_xk + *dist_ky); }
                    }
                }
            }
        }
        if dist_x.is_empty() {
            to_remove.push(*x);
        }
    }
    for x in to_remove {
        adj.0.remove(&x);
    }
}
//fn reduce( adj: &mut AdjacencyMatrix, item: &char ) {
    //let dist_k = adj.0.get(item);
    //if dist_k.is_none() {
        //return;
    //}
    //let dist_k = dist_k.unwrap().clone();
    //let mut to_remove = Vec::new();
    
    //for (x, dist_kx) in dist_k.iter() {
        //for (y, dist_ky) in dist_k.iter() {
            //if x != y {
                //match adj.0.get_mut(x).map(|dist_x| dist_x.get_mut(y)) {
                    //Some(Some(dist_xy)) => { *dist_xy = std::cmp::min(*dist_xy, *dist_kx + *dist_ky); },
                    //_ => { adj.0.entry(*x).or_insert(BTreeMap::new()).insert(*y, *dist_kx + *dist_ky); },
                //}
                //match adj.0.get_mut(y).map(|dist_y| dist_y.get_mut(x)) {
                    //Some(Some(dist_yx)) => { *dist_yx = std::cmp::min(*dist_yx, *dist_kx + *dist_ky); },
                    //_ => { adj.0.entry(*y).or_insert(BTreeMap::new()).insert(*x, *dist_kx + *dist_ky); },
                //}
            //}
        //}
    //}
    
    //for (x, dist_x) in adj.0.iter_mut() {
        //if dist_x.is_empty() {
            //to_remove.push(*x);
        //}
    //}
    //for x in to_remove {
        //adj.0.remove(&x);
    //}
//}

// This object represents the current position and the adjacency matrix
// of the remaining items in the map.
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Node {
    position: char,
    adj: AdjacencyMatrix,
}

struct WalkableWithKeys<'a>(&'a Grid);

impl<'a> Graph<Node> for WalkableWithKeys<'a> {
    fn edges( &self, node: &Node ) -> Vec<(Node, usize)> {
        let mut v = Vec::new();
        
        let reachable = node.adj.0.get(&node.position);
        if reachable.is_none() {
            return v;
        }
        let reachable = reachable.unwrap();

        // iterate over the keys reachable from the current position
        for (p, distance) in reachable.iter() {
            if p.is_ascii_lowercase() {
                let mut adj = node.adj.clone();
                adj.0.remove(&node.position);

                // open the door associated to the key we are moving to
                reduce(&mut adj, &p.to_ascii_uppercase());
                adj.0.remove(&p.to_ascii_uppercase());
                reduce(&mut adj, p);
                v.push((Node{ position: *p, adj }, *distance));
            }
        }
        v
    }
}

// This object represents the position of the four robots and the
// adjacency matrix of the remaining items in the map.
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Node4 {
    position: [char; 4],
    adj: AdjacencyMatrix,
}

struct Walkable4WithKeys<'a>(&'a Grid);

impl<'a> Graph<Node4> for Walkable4WithKeys<'a> {
    fn edges( &self, node: &Node4 ) -> Vec<(Node4, usize)> {
        let mut v = Vec::new();

        for i in 0..4 {
            let reachable = node.adj.0.get(&node.position[i]);
            if reachable.is_none() {
                continue;
            }
            let reachable = reachable.unwrap();

            // iterate over the keys reachable from robot i
            for (p, distance) in reachable.iter() {
                if p.is_ascii_lowercase() {
                    let mut position = node.position;
                    let mut adj = node.adj.clone();
                    adj.0.remove(&node.position[i]);

                    // open the door associated to the key we are moving to
                    reduce(&mut adj, &p.to_ascii_uppercase());
                    adj.0.remove(&p.to_ascii_uppercase());
                    reduce(&mut adj, p);
                    position[i] = *p;
                    v.push((Node4{ position, adj }, *distance));
                }
            }
        }
        v
    }
}

// This object represents a graph over `Node`s.
//struct WalkableGridWithKeys(HashMap<char, Vec<(char, usize)>>);

//impl Graph<Node> for WalkableGridWithKeys {
    //fn edges( &self, node: &Node ) -> Vec<(Node, usize)> {
        //let mut v = Vec::with_capacity(16);
        
        //for (next_position, d) in self.0[&node.position].iter() {
            //let mut keys = node.keys;
            //if next_position.is_ascii_uppercase() {
                //// cannot proceed through a door without the key
                //if node.keys.has_key(&next_position.to_ascii_lowercase()) {
                    //v.push((Node{ position: *next_position, keys }, *d));
                //} 
            //} else if next_position.is_ascii_lowercase() {
                //// pick up the key
                //keys.add_key(next_position);
                //v.push((Node{ position: *next_position, keys }, *d));
            //} else {
                //v.push((Node{ position: *next_position, keys }, *d));
            //}
        //}
        //v
    //}
//}

// This object represents the four positions on the grid (either on a
// key or a door) and the keys carried.
//#[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
//struct Node4 {
    //position: [char; 4],
    //keys: Keys,
//}

//// This object will represent a graph over `Node4`s.
//struct WalkableGrid4WithKeys(HashMap<char, Vec<(char, usize)>>);

//impl Graph<Node4> for WalkableGrid4WithKeys {
    //fn edges( &self, node: &Node4 ) -> Vec<(Node4, usize)>  {
        //let mut v = Vec::with_capacity(16);

        //for i in 0..4 {
            //for (next_position, d) in self.0[&node.position[i]].iter() {
                //let mut position = node.position;
                //position[i] = *next_position;
                //let mut keys = node.keys;
                //if next_position.is_ascii_uppercase() {
                    //// cannot proceed through a door without the key
                    //if node.keys.has_key(&next_position.to_ascii_lowercase()) {
                        //v.push((Node4{ position, keys }, *d));
                    //}
                //} else if next_position.is_ascii_lowercase() {
                    //// pick up the key
                    //keys.add_key(next_position);
                    //v.push((Node4{ position, keys }, *d));
                //} else {
                    //v.push((Node4{ position, keys }, *d));
                //}
            //}
        //}
        //v
    //}
//}

fn solve( input: &str ) -> (usize, usize) {
    let area = Grid::from_str(input);
    println!("{}", area);

    // compute the adjacency matrix
    let mut adj = adjacency_matrix(&area);

    //let distances = graph.bfs_paths(entrance);
    
    //for (key, value) in matrix.iter() {
    //    println!("Reachable from {}: {:?}", key, value);
    //}

    // set the starting node
    reduce(&mut adj, &'@');
    let start = Node{
        position: '@',
        adj: adj,
    };

    // compute the shortest paths in the graph of `Node`s
    let graph = WalkableWithKeys(&area);
    let distances = graph.bfs_paths(start);
    println!("States visited: {}", distances.len());
    
    // minimum distance to a terminal node (a node with all keys collected)
    let min_distance = distances
        .iter()
        .filter(|(n, _)| n.adj.0.is_empty())
        .inspect(|x| println!("{:?}", x))
        .map(|(_, d)| *d)
        .min()
        .unwrap();

    // change the map, splitting it into four
    let mut area = Grid::from_str(input);
    let entrance = *area.find(&'@').unwrap();
    area.insert(entrance, '#');
    area.insert(entrance.go(Up), '#');
    area.insert(entrance.go(Down), '#');
    area.insert(entrance.go(Left), '#');
    area.insert(entrance.go(Right), '#');
    area.insert(entrance.go(Up).go(Left), '1');
    area.insert(entrance.go(Up).go(Right), '2');
    area.insert(entrance.go(Down).go(Left), '3');
    area.insert(entrance.go(Down).go(Right), '4');
    println!("{}", area);

    // compute the adjacency matrix
    let mut adj = adjacency_matrix(&area);

    // set the starting node
    reduce(&mut adj, &'1');
    reduce(&mut adj, &'2');
    reduce(&mut adj, &'3');
    reduce(&mut adj, &'4');
    let start4 = Node4{
        position: ['1', '2', '3', '4'],
        adj: adj,
    };
    
    // compute the shortest paths in the graph of `Node`s
    let graph = Walkable4WithKeys(&area);
    let distances4 = graph.bfs_paths(start4);
    println!("States visited: {}", distances4.len());

    // minimum distance to a terminal node (a node with all keys collected)
    let min_distance4 = distances4
        .iter()
        .filter(|(n, _)| n.adj.0.is_empty())
        .inspect(|x| println!("{:?}", x))
        .map(|(_, d)| *d)
        .min()
        .unwrap();

    (min_distance, min_distance4)
}

fn main() {
    let input = std::fs::read_to_string("input/18.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}
