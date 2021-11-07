use std::collections::HashMap;

struct OrbitMap {
    parents: HashMap<String, String>,
}

impl OrbitMap {
    fn create_from(s: &str) -> OrbitMap {
        let mut parents: HashMap<String, String> = HashMap::new();

        for line in s.lines() {
            let (o1, o2) = line.split_once(')').unwrap();
            parents.entry(o2.to_string()).or_insert(o1.to_string());
        }

        OrbitMap { parents }
    }

    // computes the tree depth of an object (the steps from the root),
    // using a cache to store intermediate result.
    fn depth(&self, obj: &str, cache: &mut HashMap<String, usize>) -> usize {
        match cache.get(obj) {
            Some(&cached_depth) => cached_depth,
            None => match self.parents.get(obj) {
                None => {
                    cache.insert(obj.to_string(), 0);
                    0
                }
                Some(parent_obj) => {
                    let d = self.depth(parent_obj, cache) + 1;
                    cache.insert(obj.to_string(), d);
                    d
                }
            },
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let om = OrbitMap::create_from(&input);

    let mut cache = HashMap::new();
    let total_orbits = om.parents.keys().map(|obj| om.depth(obj, &mut cache)).sum();

    cache.clear();
    let parents_of_you = om.depth("YOU", &mut cache);
    let parents_of_san = om.depth("SAN", &mut cache);
    let parents_visited = cache.len() - 2;
    let transfer_length = (2 * parents_visited) - (parents_of_you + parents_of_san);

    (total_orbits, transfer_length)
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
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
        assert_eq!(
            solve("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n"),
            (42, 0)
        );
    }

    #[test]
    fn example02() {
        assert_eq!(
            solve("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n"),
            (54, 4)
        );
    }
}
