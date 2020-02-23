fn fuel_for_mass(m: i64) -> i64 {
    m / 3 - 2
}

fn fuel_for_mass_and_fuel(mut m: i64) -> i64 {
    let mut total = 0;
    loop {
        m = fuel_for_mass(m);
        if m <= 0 {
            break total;
        }
        total += m;
    }
}

fn solve(input: &str) -> (i64, i64) {
    let v: Vec<_> = input.lines().map(|s| s.parse::<i64>().unwrap()).collect();

    let fuel_for_mass = v.iter().map(|&m| fuel_for_mass(m)).sum::<i64>();
    let fuel_for_mass_and_fuel = v.iter().map(|&m| fuel_for_mass_and_fuel(m)).sum::<i64>();

    (fuel_for_mass, fuel_for_mass_and_fuel)
}

fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
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
        assert_eq!(solve("12"), (2, 2));
    }

    #[test]
    fn example02() {
        assert_eq!(solve("14"), (2, 2));
    }

    #[test]
    fn example03() {
        assert_eq!(solve("1969"), (654, 966));
    }

    #[test]
    fn example04() {
        assert_eq!(solve("100756"), (33583, 50346));
    }
}
