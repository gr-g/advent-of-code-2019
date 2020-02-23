// A shuffle over a deck of `p` cards is represented by an affine
// transformation of the modular group `Z_p`:
//   n -> a*n+b (mod p)
// In other words, the card with position `n` before the shuffle has
// position `a*n+b (mod p)` after the shuffle.
// Composing shuffles corresponds to composing affine transformations,
// so the result is again a shuffle with new values of `a`, `b`.
struct Shuffle {
    p: i64,
    a: i64,
    b: i64,
}

impl Shuffle {
    // identity shuffle, leaving positions unchanged
    fn id(p: i64) -> Shuffle {
        Shuffle { p, a: 1, b: 0 }
    }

    fn create_from(p: i64, input: &str) -> Shuffle {
        let mut shuffle = Shuffle::id(p);
        for s in input.lines() {
            if s.starts_with("deal into new stack") {
                shuffle.compose(-1, -1);
            } else if s.starts_with("cut ") {
                shuffle.compose(1, -s[4..].parse::<i64>().unwrap());
            } else if s.starts_with("deal with increment ") {
                shuffle.compose(s[20..].parse::<i64>().unwrap(), 0);
            } else {
                panic!("unknown shuffling technique: {}", s);
            }
        }
        shuffle
    }

    fn compose(&mut self, other_a: i64, other_b: i64) {
        self.a = (self.a as i128 * other_a as i128).rem_euclid(self.p as i128) as i64;
        self.b = (self.b as i128 * other_a as i128 + other_b as i128).rem_euclid(self.p as i128) as i64;
    }

    fn square(&mut self) {
        self.compose(self.a, self.b);
    }

    // To repeat a shuffle many times, use an "exponentiation by squaring" approach.
    // Note that this could be made more efficient by keeping `a` and `b` in Montgomery
    // form (i.e. as `aR mod p` and `bR mod p`, where `R` is 2^64, and implementing
    // the `compose` operation using Montgomery multiplication.
    fn repeat(&mut self, times: i64) {
        assert!(times > 0);
        if times == 1 {
            // done
        } else if times % 2 == 0 {
            self.repeat(times / 2);
            self.square();
        } else {
            let a = self.a;
            let b = self.b;
            self.repeat((times - 1) / 2);
            self.square();
            self.compose(a, b);
        }
    }

    fn apply(&self, n: i64) -> i64 {
        (self.a * n + self.b).rem_euclid(self.p)
    }
}

fn solve(input: &str) -> (i64, i64) {
    // Find where the card in position 2019 goes
    // after one application of the shuffle in input.
    let p = 10007;
    let shuffle = Shuffle::create_from(p, input);
    let new_position_2019 = shuffle.apply(2019);

    // Find the original position of the card that goes in position 2020
    // after `r` applications of the shuffle in input.
    // This can be done by:
    //  1. computing the inverse of the shuffle in input (finding
    //     `a`, `b` such that the shuffle in input composed with `a`, `b`
    //     is the identity - this requires computing a modular inverse),
    //     and then repeating it `r` times, or
    //  2. using the fact that `p` is prime, from Fermat's little theorem
    //     it can be proved that repeating a shuffle `p-1` times returns
    //     the deck to the initial position (for all `a` except 1), so we
    //     repeat the shuffle p-1-r times.
    // We take the second approach so we don't need a function for the
    // modular inverse.
    let p = 119315717514047;
    let r = 101741582076661;
    let mut shuffle = Shuffle::create_from(p, input);
    shuffle.repeat(p - 1 - r);
    let prev_position_2020 = shuffle.apply(2020);

    (new_position_2019, prev_position_2020)
}

fn main() {
    let input = std::fs::read_to_string("input/22.txt").unwrap();
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
        let shuffle = Shuffle::create_from(
            10,
            "\
deal with increment 7
deal into new stack
deal into new stack",
        );
        let v = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        assert_eq!(
            (0..10).map(|n| shuffle.apply(v[n])).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn example02() {
        let shuffle = Shuffle::create_from(
            10,
            "\
cut 6
deal with increment 7
deal into new stack",
        );
        let v = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        assert_eq!(
            (0..10).map(|n| shuffle.apply(v[n])).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn example03() {
        let shuffle = Shuffle::create_from(
            10,
            "\
deal with increment 7
deal with increment 9
cut -2",
        );
        let v = [6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        assert_eq!(
            (0..10).map(|n| shuffle.apply(v[n])).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn example04() {
        let shuffle = Shuffle::create_from(
            10,
            "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
        );
        let v = [9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        assert_eq!(
            (0..10).map(|n| shuffle.apply(v[n])).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
