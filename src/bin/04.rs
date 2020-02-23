fn to_six_digits(n: u32) -> [u32; 6] {
    [
        n / 100_000 % 10,
        n / 10_000 % 10,
        n / 1_000 % 10,
        n / 100 % 10,
        n / 10 % 10,
        n % 10,
    ]
}

fn is_valid1(p: &[u32; 6]) -> bool {
    let mut dup = false;
    let mut inc = true;

    for i in 0..5 {
        if p[i + 1] < p[i] {
            inc = false;
        }
        if p[i + 1] == p[i] {
            dup = true;
        }
    }

    dup && inc
}

fn is_valid2(p: &[u32; 6]) -> bool {
    let mut dup = false;
    let mut inc = true;

    for i in 0..5 {
        if p[i + 1] < p[i] {
            inc = false;
        }
    }
    if (p[1] == p[0] && p[2] != p[1]) || (p[5] == p[4] && p[4] != p[3]) {
        dup = true;
    } else {
        for i in 1..4 {
            if (p[i + 1] == p[i]) && (p[i] != p[i - 1]) && (p[i + 2] != p[i + 1]) {
                dup = true;
            }
        }
    }

    dup && inc
}

fn solve(input_start: u32, input_end: u32) -> (usize, usize) {
    let n_valid1 = (input_start..=input_end)
        .filter(|&p| is_valid1(&to_six_digits(p)))
        .count();
    let n_valid2 = (input_start..=input_end)
        .filter(|&p| is_valid2(&to_six_digits(p)))
        .count();

    (n_valid1, n_valid2)
}

fn main() {
    let now = std::time::Instant::now();
    let s = solve(234208, 765869);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(is_valid1(&to_six_digits(111111)), true);
    }

    #[test]
    fn example02() {
        assert_eq!(is_valid2(&to_six_digits(111111)), false);
    }

    #[test]
    fn example03() {
        assert_eq!(is_valid1(&to_six_digits(223450)), false);
    }

    #[test]
    fn example04() {
        assert_eq!(is_valid2(&to_six_digits(223450)), false);
    }

    #[test]
    fn example05() {
        assert_eq!(is_valid1(&to_six_digits(123789)), false);
    }

    #[test]
    fn example06() {
        assert_eq!(is_valid2(&to_six_digits(123789)), false);
    }

    #[test]
    fn example07() {
        assert_eq!(is_valid1(&to_six_digits(112233)), true);
    }

    #[test]
    fn example08() {
        assert_eq!(is_valid2(&to_six_digits(112233)), true);
    }

    #[test]
    fn example09() {
        assert_eq!(is_valid1(&to_six_digits(123444)), true);
    }

    #[test]
    fn example10() {
        assert_eq!(is_valid2(&to_six_digits(123444)), false);
    }

    #[test]
    fn example11() {
        assert_eq!(is_valid1(&to_six_digits(111122)), true);
    }

    #[test]
    fn example12() {
        assert_eq!(is_valid2(&to_six_digits(111122)), true);
    }
}
