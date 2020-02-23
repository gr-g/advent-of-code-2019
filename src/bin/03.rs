use std::collections::HashMap;

struct Wire {
    tip: (i64,i64),
    len: i64,
    path: HashMap<(i64,i64), i64>, // locations visited and time of first visit
}
 
impl Wire {
    fn from_str( s: &str ) -> Wire {
        let mut w = Wire { tip: (0, 0), len: 0, path: HashMap::new() };
        for m in s.split(',') {
            let (d, n) = m.split_at(1);
            let n = n.parse::<i64>().unwrap();
            match d {
                "R" => for _ in 0..n { w.len += 1; w.tip.0 += 1; w.path.entry(w.tip).or_insert(w.len); },
                "L" => for _ in 0..n { w.len += 1; w.tip.0 -= 1; w.path.entry(w.tip).or_insert(w.len); },
                "U" => for _ in 0..n { w.len += 1; w.tip.1 += 1; w.path.entry(w.tip).or_insert(w.len); },
                "D" => for _ in 0..n { w.len += 1; w.tip.1 -= 1; w.path.entry(w.tip).or_insert(w.len); },
                _ => panic!(),
            }
        }
        w
    }
}

fn solve( input: &str ) -> (i64, i64) {
    let input_lines: Vec<_> = input.lines().collect();

    let w1 = Wire::from_str(input_lines[0]);
    let w2 = Wire::from_str(input_lines[1]);

    let close_cross = w1.path.keys()
        .filter(|&p| w2.path.contains_key(p))
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    let early_cross = w1.path.keys()
        .filter(|&p| w2.path.contains_key(p))
        .map(|p| w1.path[p] + w2.path[p])
        .min()
        .unwrap();

    (close_cross, early_cross)
}

fn main() {
    let input = std::fs::read_to_string("input/03.txt").unwrap();
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
        assert_eq!(solve("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), (159,610));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), (135,410));
    }
}
