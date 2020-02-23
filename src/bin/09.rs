use advent_of_code_2019::intcode::IntCode;

fn solve(input: &str) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut c = IntCode::new(&program);

    c.input.push_back(1);
    c.run();
    let output_with_1 = c.output.pop().unwrap();

    c.reset(&program);
    c.input.push_back(2);
    c.run();
    let output_with_2 = c.output.pop().unwrap();

    (output_with_1, output_with_2)
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
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
        let mut c = IntCode::new(&[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
        c.run();
        assert_eq!(
            c.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn example02() {
        let mut c = IntCode::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        c.run();
        assert_eq!(c.output, vec![1219070632396864]);
    }

    #[test]
    fn example03() {
        let mut c = IntCode::new(&[104, 1125899906842624, 99]);
        c.run();
        assert_eq!(c.output, vec![1125899906842624]);
    }
}
