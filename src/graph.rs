use std::cmp::Ordering;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

// A trait for structures that represent unweighted graphs
// with nodes of type `N`.
pub trait UnweightedGraph<N> where N: Eq + Hash + Clone {
    fn edges( &self, node: &N ) -> Vec<N>;

    // Shortest paths computed by breadth-first search.
    fn shortest_paths( &self, start: N, targets: &[N] ) -> HashMap<N, usize> {
        let mut distances = HashMap::new();
        let mut frontier = VecDeque::new();

        frontier.push_back(start.clone());
        distances.insert(start, 0);

        while let Some(node) = frontier.pop_front() {
            let distance = distances[&node];
            if targets.contains(&node) {
                break;
            }
            for n in self.edges(&node) {
                if let Entry::Vacant(e) = distances.entry(n) {
                    frontier.push_back(e.key().clone());
                    e.insert(distance + 1);
                }
            }
        }
        distances
    }
}

// A trait for structures that represent graphs with nodes of type `N`
// with positive weights (distances) associated to the edges.
pub trait Graph<N> where N: Ord + Hash + Clone {
    fn edges( &self, node: &N ) -> Vec<(N, usize)>;

    // Scans a graph using breadth-first search, recording the distance to each node.
    // This is guaranteed to give the shortest path when the graph is such that when
    // two paths reach the same node in a different number of steps, the path with
    // fewer steps is always better.
    fn bfs_paths( &self, start: N ) -> HashMap<N, usize> {
        let mut distances = HashMap::new();
        let mut frontier = VecDeque::new();

        frontier.push_back(start.clone());
        distances.insert(start, 0);

        while let Some(node) = frontier.pop_front() {
            let distance = distances[&node];
            for (n, w) in self.edges(&node) {
                match distances.entry(n) {
                    Entry::Vacant(e) => {
                        frontier.push_back(e.key().clone());
                        e.insert(distance + w);
                    },
                    Entry::Occupied(mut e) if *e.get() > distance + w => {
                        e.insert(distance + w);
                    },
                    _ => {},
                }
            }
        }
        distances
    }

    // Implementation of Dijkstra's algorithm for shortest paths,
    // using a binary heap.
    fn shortest_paths( &self, start: N ) -> HashMap<N, usize> {
        let mut distances = HashMap::new();
        let mut frontier = BinaryHeap::new();

        distances.insert(start.clone(), 0);
        frontier.push(FrontierNode{ node: start, distance: 0 });

        while let Some(FrontierNode{ node, distance }) = frontier.pop() {
            if distances.get(&node).filter(|d| **d < distance).is_some() {
                continue;
            }

            for (n, w) in self.edges(&node) {
                let next = FrontierNode{ node: n, distance: distance + w };
                match distances.entry(next.node.clone()) {
                    Entry::Occupied(e) if *e.get() <= next.distance => {
                        continue;
                    },
                    Entry::Occupied(mut e) => {
                        e.insert(next.distance);
                        frontier.push(next);
                    },
                    Entry::Vacant(e) => {
                        e.insert(next.distance);
                        frontier.push(next);
                    },
                }
            }
        }
        distances
    }
}

#[derive(Eq, PartialEq)]
struct FrontierNode<T> {
    node: T,
    distance: usize,
}

// Implement `Ord` so that the "maximum" frontier node is the one with
// the smallest distance. This makes it possible to use a BinaryHeap
// as min-priority queue for Dijkstra's algorithm.
impl<T> Ord for FrontierNode<T> where T: Ord {
    fn cmp( &self, other: &Self ) -> Ordering {
        other.distance.cmp(&self.distance)
            .then(self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for FrontierNode<T> where FrontierNode<T>: Ord {
    fn partial_cmp( &self, other: &Self ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum ExplorationStep<T> {
    Advance(T),
    Backtrack(T),
}

// A trait for structures representing "states" that can be advanced
// with "actions" towards a solution, with the possibility to backtrack.
pub trait Backtracking {
    type Action;

    // This should return the possible actions, from least likely to
    // lead to a solution to most likely to lead to a solution.
    fn list_actions( &self ) -> Vec<Self::Action>;

    // This function should return true if the action succeeded and
    // the state changed. It should return false if the state did not
    // change.
    fn try_action( &mut self, action: &Self::Action ) -> bool;

    // This should rewind an action.
    fn backtrack( &mut self, action: &Self::Action );

    // This should return true if the current state is a solution.
    fn is_solution( &self ) -> bool;

    // Explores the state space using DFS/backtracking, looking for a state
    // that satisfies `is_solution`. If a solution is found, returns the
    // sequence of winning actions.
    fn explore( &mut self ) -> Option<Vec<Self::Action>> {
        let mut stack = Vec::new();

        loop {
            // check if the current state is a solution
            if self.is_solution() {
                let winning_actions = stack.into_iter().filter_map(|s| {
                    match s {
                        ExplorationStep::Advance(_) => None,
                        ExplorationStep::Backtrack(a) => Some(a),
                    }
                }).collect();
                return Some(winning_actions);
            }

            // consider the possible next actions
            for a in self.list_actions() {
                stack.push(ExplorationStep::Advance(a));
            }

            // look for a new state to visit
            loop {
                match stack.pop() {
                    None => {
                        // no more actions to try, no solutions found
                        return None;
                    }
                    Some(ExplorationStep::Backtrack(a)) => {
                        // go back to a previous state
                        self.backtrack(&a);
                        continue;
                    }
                    Some(ExplorationStep::Advance(a)) => {
                        // try this action
                        if !self.try_action(&a) {
                            continue;
                        }

                        // an action succeeded, we are now in a new state
                        stack.push(ExplorationStep::Backtrack(a));
                        break;
                    }
                }
            }
        }
    }
}
