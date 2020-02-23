use std::cmp::Ordering;

fn gcd(mut m: usize, mut n: usize) -> usize {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n
}

fn lcm(m: usize, n: usize) -> usize {
    m * n / gcd(m, n)
}

#[derive(Debug, Clone)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn create_from(s: &str) -> Moon {
        let mut values = s[1..s.len() - 1]
            .split(", ")
            .map(|e| e[2..].parse::<i64>().unwrap());
        Moon {
            pos: [
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            ],
            vel: [0, 0, 0],
        }
    }

    fn energy(&self) -> i64 {
        (self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs())
            * (self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs())
    }
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            for d in 0..3 {
                match moons[i].pos[d].cmp(&moons[j].pos[d]) {
                    Ordering::Less => { moons[i].vel[d] += 1; moons[j].vel[d] -= 1; },
                    Ordering::Greater => { moons[i].vel[d] -= 1; moons[j].vel[d] += 1; },
                    Ordering::Equal => {},
                }
            }
        }
    }
    for i in 0..moons.len() {
        for d in 0..3 {
            moons[i].pos[d] += moons[i].vel[d];
        }
    }
}

fn simulate(moons: &mut [Moon], steps: usize) -> (i64, usize) {
    let moons0 = moons.to_vec();
    let mut min_cycle_axis = [None, None, None];
    let mut target_energy = 0;

    'sim: for n in 1.. {
        step(moons);

        for d in 0..3 {
            if min_cycle_axis[d].is_none()
                && moons
                    .iter()
                    .map(|m| (m.pos[d], m.vel[d]))
                    .eq(moons0.iter().map(|m| (m.pos[d], m.vel[d])))
            {
                //println!("Repeated coord {} after {} steps.", d, n);
                min_cycle_axis[d] = Some(n);
                if min_cycle_axis.iter().all(|&x| x.is_some()) {
                    break 'sim;
                }
            }
        }

        if n == steps {
            target_energy = moons.iter().map(Moon::energy).sum();
            println!("Energy after {} steps: {}", n, target_energy);
        }
    }

    let min_cycle = lcm(
        lcm(min_cycle_axis[0].unwrap(), min_cycle_axis[1].unwrap()),
        min_cycle_axis[2].unwrap(),
    );

    (target_energy, min_cycle)
}

fn solve(input: &str, steps: usize) -> (i64, usize) {
    let mut lines = input.lines();

    let mut system = [
        Moon::create_from(lines.next().unwrap()),
        Moon::create_from(lines.next().unwrap()),
        Moon::create_from(lines.next().unwrap()),
        Moon::create_from(lines.next().unwrap()),
    ];

    simulate(&mut system, steps)
}

fn main() {
    let input = std::fs::read_to_string("input/12.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 1000);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(solve("\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>", 10), (179, 2772));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>", 100), (1940, 4686774924));
    }
}
