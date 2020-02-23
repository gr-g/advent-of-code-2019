// The main part of the FFT computation (before taking the least significant
// digit) corresponds to a linear operation Pv, where P is the nxn matrix
// with the repeating patterns on the rows.
// P has O(n^2) non-zero elements, but the 'derivative' of the pattern has
// only O(n log(n)) non-zero elements. This can be exploited to compute Pv
// in O(n log(n)) time by doing:
//
//   P v = P D inv(D) v
//
// where D is the 'discrete derivative' operator:
//     | 1 -1  0  0  0 ... 0  0 |
//     | 0  1 -1  0  0 ... 0  0 |
// D = | 0  0  1 -1  0 ... 0  0 |
//     | ...                    |
//     | 0  0  0  0  0 ... 1 -1 |
//     | 0  0  0  0  0 ... 0  1 |
//
// and inv(D) is the 'discrete integral' operator:
//          | 1  1  1  1  1 ... 1  1 |
//          | 0  1  1  1  1 ... 1  1 |
// inv(D) = | 0  0  1  1  1 ... 1  1 |
//          | ...                    |
//          | 0  0  0  0  0 ... 1  1 |
//          | 0  0  0  0  0 ... 0  1 |
//
// Applying inv(D) is simply taking a cumulative sum. It is done in-place
// in n steps.
//
// The operator PD has O(n log(n)) non-zero elements and can be applied
// in-place thanks to the upper-triangular structure:
//       | 1 -1 -1  1  1 -1 -1  1  1 -1 -1  1 ... |
//       | 0  1  0 -1  0 -1  0  1  0  1  0 -1 ... |
// P D = | 0  0  1  0  0 -1  0  0 -1  0  0  1 ... |
//       | 0  0  0  1  0  0  0 -1  0  0  0 -1 ... |
//       | ...                                    |

// This function applies a number of FFT phases to the tail of the input
// vector (from offset to the end of the vector).
fn fft(v: &mut [i64], phases: usize, offset: usize) {
    let n = v.len();

    for _ in 0..phases {
        // v := inv(D) v
        for i in (offset..n - 1).rev() {
            v[i] += v[i + 1]
        }

        // v := PD v,
        for i in offset..n / 2 {
            let step = i + 1;
            let mut j = i;
            loop {
                j += step; if j >= n { break; }; v[i] -= v[j];
                j += step; if j >= n { break; }; v[i] -= v[j];
                j += step; if j >= n { break; }; v[i] += v[j];
                j += step; if j >= n { break; }; v[i] += v[j];
            }
        }

        // final result
        for i in offset..n {
            v[i] = v[i].abs() % 10;
        }
    }
}

fn solve(input: &str) -> (String, String) {
    let mut signal: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    fft(&mut signal, 100, 0);

    let message = signal[0..8]
        .iter()
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .collect();

    let offset = input[0..7].parse().unwrap();

    let mut real_signal: Vec<_> = input
        .trim()
        .repeat(10000)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    if offset > real_signal.len() - 8 {
        return (message, "".to_string());
    }

    fft(&mut real_signal, 100, offset);

    let real_message = real_signal[offset..offset + 8]
        .iter()
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .collect();

    (message, real_message)
}

fn main() {
    let input = std::fs::read_to_string("input/16.txt").unwrap();
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
        let mut v = [1, 2, 3, 4, 5, 6, 7, 8];
        fft(&mut v, 1, 0);
        assert_eq!(v, [4, 8, 2, 2, 6, 1, 5, 8]);
        fft(&mut v, 1, 0);
        assert_eq!(v, [3, 4, 0, 4, 0, 4, 3, 8]);
        fft(&mut v, 1, 0);
        assert_eq!(v, [0, 3, 4, 1, 5, 5, 1, 8]);
        fft(&mut v, 1, 0);
        assert_eq!(v, [0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn example02() {
        assert_eq!(&solve("80871224585914546619083218645595").0, "24176176");
    }

    #[test]
    fn example03() {
        assert_eq!(&solve("19617804207202209144916044189917").0, "73745418");
    }

    #[test]
    fn example04() {
        assert_eq!(&solve("69317163492948606335995924319873").0, "52432133");
    }

    #[test]
    fn example05() {
        assert_eq!(&solve("03036732577212944063491565474664").1, "84462026");
    }

    #[test]
    fn example06() {
        assert_eq!(&solve("02935109699940807407585447034323").1, "78725270");
    }

    #[test]
    fn example07() {
        assert_eq!(&solve("03081770884921959731165446850517").1, "53553731");
    }
}
