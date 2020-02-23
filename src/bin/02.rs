use advent_of_code_2019::intcode::IntCode;

fn run_with_args( computer: &mut IntCode, w1: i64, w2: i64 ) -> i64 {
    computer.memory[1] = w1;
    computer.memory[2] = w2;
    computer.run();
    computer.memory[0]
}

fn solve( input: &str, target: i64 ) -> (i64, Option<i64>) {
    let program: Vec<_> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();
    let mut c = IntCode::new(&program);
    
    let output = run_with_args(&mut c, 12, 2);
    
    let mut args = None;
    'search:
    for i in 0..100 {
        for j in 0..100 {
            c.reset(&program);
            if run_with_args(&mut c, i, j) == target {
                args = Some(i*100 + j);
                break 'search;
            }
        }  
    }

    (output, args)
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input, 19690720);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example01() {
        let mut c = IntCode::new(&[1,9,10,3,2,3,11,0,99,30,40,50]);
        c.run();
        assert_eq!(&c.memory, &[3500,9,10,70,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn example02() {
        let mut c = IntCode::new(&[1,0,0,0,99]);
        c.run();
        assert_eq!(&c.memory, &[2,0,0,0,99]);
    }

    #[test]
    fn example03() {
        let mut c = IntCode::new(&[2,3,0,3,99]);
        c.run();
        assert_eq!(&c.memory, &[2,3,0,6,99]);
    }

    #[test]
    fn example04() {
        let mut c = IntCode::new(&[2,4,4,5,99,0]);
        c.run();
        assert_eq!(&c.memory, &[2,4,4,5,99,9801]);
    }

    #[test]
    fn example05() {
        let mut c = IntCode::new(&[1,1,1,4,99,5,6,0,99]);
        c.run();
        assert_eq!(&c.memory, &[30,1,1,4,2,5,6,0,99]);
    }
}
