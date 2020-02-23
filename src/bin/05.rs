use advent_of_code_2019::intcode::IntCode;

fn solve( input: &str ) -> (i64, i64) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut c = IntCode::new(&program);

    c.input.push_back(1);
    c.run();
    let output_with_1 = c.output.pop().unwrap();

    c.reset(&program);
    c.input.push_back(5);
    c.run();
    let output_with_5 = c.output.pop().unwrap();
    
    (output_with_1, output_with_5)
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();
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
        let mut c = IntCode::new(&[3,0,4,0,99]);
        c.input.push_back(123); c.run();
        assert_eq!(c.output, vec![123]);
    }

    #[test]
    fn example02() {
        let mut c = IntCode::new(&[1002,4,3,4,33]);
        c.run();
        assert_eq!(&c.memory, &[1002,4,3,4,99]);
    }

    #[test]
    fn example03() {
        let mut c = IntCode::new(&[3,9,8,9,10,9,4,9,99,-1,8]);
        c.input.push_back(7); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example04() {
        let mut c = IntCode::new(&[3,9,8,9,10,9,4,9,99,-1,8]);
        c.input.push_back(8); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example05() {
        let mut c = IntCode::new(&[3,9,8,9,10,9,4,9,99,-1,8]);
        c.input.push_back(9); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example06() {
        let mut c = IntCode::new(&[3,9,7,9,10,9,4,9,99,-1,8]);
        c.input.push_back(7); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example07() {
        let mut c = IntCode::new(&[3,9,7,9,10,9,4,9,99,-1,8]);
        c.input.push_back(8); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example08() {
        let mut c = IntCode::new(&[3,9,7,9,10,9,4,9,99,-1,8]);
        c.input.push_back(9); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example09() {
        let mut c = IntCode::new(&[3,3,1108,-1,8,3,4,3,99]);
        c.input.push_back(7); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example10() {
        let mut c = IntCode::new(&[3,3,1108,-1,8,3,4,3,99]);
        c.input.push_back(8); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example11() {
        let mut c = IntCode::new(&[3,3,1108,-1,8,3,4,3,99]);
        c.input.push_back(9); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example12() {
        let mut c = IntCode::new(&[3,3,1107,-1,8,3,4,3,99]);
        c.input.push_back(7); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example13() {
        let mut c = IntCode::new(&[3,3,1107,-1,8,3,4,3,99]);
        c.input.push_back(8); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example14() {
        let mut c = IntCode::new(&[3,3,1107,-1,8,3,4,3,99]);
        c.input.push_back(9); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example15() {
        let mut c = IntCode::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        c.input.push_back(0); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example16() {
        let mut c = IntCode::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        c.input.push_back(1); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example17() {
        let mut c = IntCode::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]);
        c.input.push_back(2); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example18() {
        let mut c = IntCode::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        c.input.push_back(0); c.run();
        assert_eq!(c.output, vec![0]);
    }

    #[test]
    fn example19() {
        let mut c = IntCode::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        c.input.push_back(1); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example20() {
        let mut c = IntCode::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1]);
        c.input.push_back(2); c.run();
        assert_eq!(c.output, vec![1]);
    }

    #[test]
    fn example21() {
        let mut c = IntCode::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        c.input.push_back(5); c.run();
        assert_eq!(c.output, vec![999]);
    }

    #[test]
    fn example22() {
        let mut c = IntCode::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        c.input.push_back(8); c.run();
        assert_eq!(c.output, vec![1000]);
    }

    #[test]
    fn example23() {
        let mut c = IntCode::new(&[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99]);
        c.input.push_back(15); c.run();
        assert_eq!(c.output, vec![1001]);
    }
}
